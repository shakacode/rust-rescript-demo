open XmlHttpRequest

module type Query = {
  module Raw: {
    type t
    type t_variables
  }

  type t
  type t_variables

  let query: string

  let parse: Raw.t => t
  let serialize: t => Raw.t
  let serializeVariables: t_variables => Raw.t_variables

  let unsafe_fromJson: Js.Json.t => Raw.t
  let toJson: Raw.t => Js.Json.t
  let variablesToJson: Raw.t_variables => Js.Json.t
}

module Error = {
  type t<'error> = ExtendedError('error) | OpaqueFailure

  // We expect 2 types of error from api:
  // 1. Extended Error: contains valuable information for the ui
  //    ```
  //    {
  //      "errors": [
  //        {
  //          "message": "Extended Error",
  //          "extensions": {
  //            "details": {
  //              "reason": <string>,
  //              "payload": <any>
  //            }
  //          }
  //        }
  //      ]
  //    }
  //    ```
  //
  // 2. Internal Server Error: opaque error means server has failed to handle a request
  //    ```
  //    {
  //      "errors": [
  //        {
  //          "message": "Internal Server Error",
  //        }
  //      ]
  //    }
  //    ```
  let fromJson = (json, parseExtendedError) => {
    let message = json->Js.Dict.get("message")->Option.flatMap(Js.Json.decodeString)
    switch message {
    | Some("Internal Server Error") => Ok(OpaqueFailure)
    | Some("Extended Error") =>
      let extensions = json->Js.Dict.get("extensions")
      switch (extensions, parseExtendedError) {
      | (Some(extensions), Some(parseExtendedError)) =>
        switch extensions->Js.Json.decodeObject {
        | Some(extensions) =>
          let details = extensions->Js.Dict.get("details")->Option.flatMap(Js.Json.decodeObject)
          switch details {
          | Some(details) =>
            let reason = details->Js.Dict.get("reason")->Option.flatMap(Js.Json.decodeString)
            let payload = details->Js.Dict.get("payload")
            switch reason {
            | Some(reason) =>
              switch parseExtendedError(~reason, ~payload) {
              | Ok(error) => Ok(ExtendedError(error))
              | Error(error) => Error(#ParseError(error))
              }
            | None => Error(#ReasonParseError)
            }
          | None => Error(#DetailsParseError)
          }
        | None => Error(#ExtentionsParseError)
        }
      | (Some(extensions), None) => Error(#NoParserForExtendedError(extensions))
      | (None, Some(_) | None) => Error(#NoExtensionsOnExtendedError)
      }
    | Some(message) => Error(#UnexpectedMessage(message))
    | None => Error(#NoMessage)
    }
  }
}

type abort = unit => unit

let exec:
  type data variables error. (
    ~query: module(Query with type t = data and type t_variables = variables),
    ~variables: variables,
    ~extendedError: option<(~reason: string, ~payload: option<Js.Json.t>) => result<error, string>>,
    result<data, Error.t<error>> => unit,
  ) => option<abort> =
  (~query, ~variables, ~extendedError, handle) => {
    %log.debug(
      "Request"
      ("Query", query)
      ("Variables", variables)
    )

    let module(Query) = query

    let xhr = XmlHttpRequest.make()

    xhr->openAsync("POST", `http://${Env.apiHost}:${Env.apiPort}${Env.apiPath}`)
    xhr->setRequestHeader("Content-Type", "application/json;charset=UTF-8")

    xhr->on(
      #readystatechange(
        () => {
          switch xhr->readyState {
          | DONE =>
            let response = xhr->response
            let status = xhr->Http.Status.fromXhr

            %log.debug(
              "Response"
              ("Status", status)
              ("Body", response)
            )

            switch status {
            | {result: #Ok, _} =>
              switch response->Js.Json.deserializeUnsafe->Js.Json.decodeObject {
              | Some(json) =>
                let data = json->Js.Dict.get("data")->Option.flatMap(data =>
                  switch data->Js.Json.decodeNull {
                  | Some(_) => None
                  | None => Some(data)
                  }
                )
                switch data {
                | Some(data) =>
                  let data = data->Query.unsafe_fromJson->Query.parse
                  %log.debug(
                    "Success"
                    ("Data", data)
                  )
                  Ok(data)->handle
                | None =>
                  let errors = json->Js.Dict.get("errors")
                  switch errors {
                  | Some(errors) =>
                    let error =
                      errors
                      ->Js.Json.decodeArray
                      ->Option.flatMap(errors => errors->Array.get(0))
                      ->Option.flatMap(Js.Json.decodeObject)
                      ->Option.map(error => error->Error.fromJson(extendedError))
                    switch error {
                    | Some(Ok(error)) =>
                      %log.debug(
                        "Failure"
                        ("Error", error)
                      )
                      Error(error)->handle
                    | Some(Error(reason)) =>
                      %log.error(
                        "Parse error failure"
                        ("Status", status)
                        ("Payload", json)
                        ("Reason", reason)
                      )
                      Error(OpaqueFailure)->handle
                    | None =>
                      %log.error(
                        "Invalid error"
                        ("Status", status)
                        ("Payload", json)
                      )
                      Error(OpaqueFailure)->handle
                    }
                  | None =>
                    %log.error(
                      "Invalid payload"
                      ("Status", status)
                      ("Payload", json)
                    )
                    Error(OpaqueFailure)->handle
                  }
                }
              | None =>
                %log.error(
                  "Non JSON payload"
                  ("Status", status)
                  ("Response", response)
                )
                Error(OpaqueFailure)->handle
              }
            | {Http.Status.result: #Redirect, _} =>
              %log.error(
                "HTTP Redirect"
                ("Status", status)
                ("Response", response)
              )
              Error(OpaqueFailure)->handle
            | {result: #ClientError, _} =>
              %log.error(
                "HTTP Client Error"
                ("Status", status)
                ("Response", response)
              )
              Error(OpaqueFailure)->handle
            | {result: #ServerError, _} =>
              %log.warn(
                "HTTP Server Error"
                ("Status", status)
                ("Response", response)
              )
              Error(OpaqueFailure)->handle
            | {result: #UnhandledError, _} =>
              %log.error(
                "HTTP Unhandled Error"
                ("Status", status)
                ("Response", response)
              )
              Error(OpaqueFailure)->handle
            }
          | UNSENT | OPENED | HEADERS_RECEIVED | LOADING => ()
          }
        },
      ),
    )

    let body = Js.Dict.empty()
    body->Js.Dict.set("query", Query.query->Js.Json.string)
    body->Js.Dict.set("variables", variables->Query.serializeVariables->Query.variablesToJson)

    let json = body->Js.Json.object_->Js.Json.stringify

    xhr->send(json)

    Some(() => xhr->abort)
  }
