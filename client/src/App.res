type state = Loading | Ready(array<Post.t>) | Failure

type action = ShowPosts(array<Post.t>) | Fail

let reducer = (_state, action) =>
  switch action {
  | ShowPosts(posts) => Ready(posts)
  | Fail => Failure
  }

@react.component
let make = () => {
  let (state, dispatch) = reducer->React.useReducer(Loading)

  React.useEffect0(() => {
    open PostQuery__GetAll
    Api.exec(~query=module(Query), ~variables=(), ~extendedError=None, res =>
      switch res {
      | Ok(res) => ShowPosts(res.posts)->dispatch
      | Error(_) => Fail->dispatch
      }
    )
  })

  <div>
    {switch state {
    | Loading => "Loading..."->React.string
    | Ready(posts) =>
      posts
      ->Array.map(post => <div key={post.id->PostId.toString}> {post.title->React.string} </div>)
      ->React.array
    | Failure => "Oh no"->React.string
    }}
  </div>
}
