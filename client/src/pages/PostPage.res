module PostView = {
  type state =
    Viewing({deletionError: option<Api.Error.t<PostMutation__Delete.ExtendedError.t>>}) | Deleting

  type action = Delete | FailDeletion({reason: Api.Error.t<PostMutation__Delete.ExtendedError.t>})

  let initialState = Viewing({deletionError: None})

  @react.component
  let make = (~post: Post.t) => {
    let (state, dispatch) = initialState->ReactUpdate.useReducer((action, _state) =>
      switch action {
      | Delete =>
        UpdateWithSideEffects(
          Deleting,
          ({state: _, send: dispatch}) => {
            open PostMutation__Delete
            Api.exec(
              ~query=module(Query),
              ~variables=Variables.make(~id=post.id),
              ~extendedError=ExtendedError.parse->Some,
              res =>
                switch res {
                | Ok(_) => Route.posts->Router.push
                | Error(error) => FailDeletion({reason: error})->dispatch
                },
            )
          },
        )
      | FailDeletion({reason}) => Update(Viewing({deletionError: Some(reason)}))
      }
    )

    <>
      <div>
        <h1> {post.title->React.string} </h1>
        <div>
          <button type_="button" onClick={_ => Route.editPost(~id=post.id)->Router.push}>
            {"Edit"->React.string}
          </button>
        </div>
        <div>
          {switch state {
          | Viewing({deletionError}) => <>
              <button type_="button" onClick={_ => Delete->dispatch}>
                {"Delete"->React.string}
              </button>
              {switch deletionError {
              | Some(error) =>
                switch error {
                | ExtendedError(PostNotFound) => <div> {"Post not found"->React.string} </div>
                | OpaqueFailure => <div> {"Something went wrong"->React.string} </div>
                }
              | None => React.null
              }}
            </>
          | Deleting =>
            <button type_="button" disabled=true> {"Deleting..."->React.string} </button>
          }}
        </div>
      </div>
      <div> {post.content->React.string} </div>
      <div> <Router.Link route={Route.posts}> {"Back to posts"->React.string} </Router.Link> </div>
    </>
  }
}

type state = Loading | Ready(Post.t) | Failure

type action = ShowPost(Post.t) | Fail

let reducer = (_state, action) =>
  switch action {
  | ShowPost(post) => Ready(post)
  | Fail => Failure
  }

@react.component
let make = (~id) => {
  let (state, dispatch) = reducer->React.useReducer(Loading)

  React.useEffect0(() => {
    open PostQuery__GetById
    Api.exec(
      ~query=module(Query),
      ~variables=Variables.make(~id),
      ~extendedError=ExtendedError.parse->Some,
      res =>
        switch res {
        | Ok(res) => ShowPost(res.post)->dispatch
        | Error(_) => Fail->dispatch
        },
    )
  })

  <div>
    {switch state {
    | Loading => "Loading..."->React.string
    | Ready(post) => <PostView post />
    | Failure => "Oh no"->React.string
    }}
  </div>
}
