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
    | Ready(post) => <>
        <div>
          <h1> {post.title->React.string} </h1>
          <div>
            <Router.Link route={Route.editPost(~id)}> {"Edit"->React.string} </Router.Link>
          </div>
        </div>
        <div> {post.content->React.string} </div>
        <div>
          <Router.Link route={Route.posts}> {"Back to posts"->React.string} </Router.Link>
        </div>
      </>
    | Failure => "Oh no"->React.string
    }}
  </div>
}
