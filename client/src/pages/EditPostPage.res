module EditorView = {
  @react.component
  let make = (~post: Post.t) => {
    let initialInput = React.useMemo0(() => {
      PostEditor.title: post.title,
      content: post.content,
    })

    <Layout.Content>
      <PostEditor
        initialInput
        cancelRoute={Route.post(~id=post.id)}
        onSubmit={(input, ~onFailure as fail) => {
          open PostMutation__Update
          Api.exec(
            ~query=module(Query),
            ~variables=Variables.make(~id=post.id, ~title=input.title, ~content=input.content),
            ~extendedError=ExtendedError.parse->Some,
            res =>
              switch res {
              | Ok(res) => Route.post(~id=res.post.id)->Router.push
              | Error(error) =>
                switch error {
                | ExtendedError(PostNotFound) => fail(~reason="Post not found", ())
                | OpaqueFailure => fail()
                }
              },
          )
        }}
      />
    </Layout.Content>
  }
}

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

  <Layout>
    <Layout.Header> <H1> {"Edit post"->React.string} </H1> </Layout.Header>
    {switch state {
    | Loading => "Loading..."->React.string
    | Ready(post) => <EditorView post />
    | Failure => "Oh no"->React.string
    }}
  </Layout>
}
