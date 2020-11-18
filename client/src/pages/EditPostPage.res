type state = Loading | Ready(Post.t) | Failure

type action = ShowPostEditor(Post.t) | Fail

let reducer = (_state, action) =>
  switch action {
  | ShowPostEditor(post) => Ready(post)
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
        | Ok(res) => ShowPostEditor(res.post)->dispatch
        | Error(_) => Fail->dispatch
        },
    )
  })

  <div>
    {switch state {
    | Loading => "Loading..."->React.string
    | Ready(post) =>
      %log.debug(post)
      <div>
        <div> <input type_="text" placeholder="Title" value={post.title} onChange=ignore /> </div>
        <div> <textarea value={post.content} onChange=ignore /> </div>
        <div> <Router.Link route={Route.post(~id)}> {"Done"->React.string} </Router.Link> </div>
      </div>
    | Failure => "Oh no"->React.string
    }}
  </div>
}
