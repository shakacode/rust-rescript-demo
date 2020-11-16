module Status = {
  type t = {
    result: [#Ok | #Redirect | #ClientError | #ServerError | #UnhandledError],
    code: int,
  }

  let fromXhr = xhr => {
    let code = xhr->XmlHttpRequest.status
    let result = if code >= 200 && code < 300 {
      #Ok
    } else if code >= 300 && code < 400 {
      #Redirect
    } else if code >= 400 && code < 500 {
      #ClientError
    } else if code >= 500 && code < 600 {
      #ServerError
    } else {
      #UnhandledError
    }
    {result: result, code: code}
  }
}
