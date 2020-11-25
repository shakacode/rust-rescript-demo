module Css = %css(
  let headerControls = css`
    display: grid;
    grid-template-columns: max-content max-content;
    grid-column-gap: 10px;
    align-items: center;
    justify-content: end;
  `

  let content = css`
    display: grid;
    grid-template-rows: max-content max-content;
    grid-row-gap: 14px;
  `

  let post = css`
    display: flex;
    flex-flow: column nowrap;
  `

  let footer = css`
    display: flex;
    flex-flow: row nowrap;
  `
)

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
      <Layout.HeaderWithControls>
        <H1> {post.title->React.string} </H1>
        <div className=Css.headerControls>
          <div>
            <Link.AsButton route={Route.editPost(~id=post.id)} size=SM style=Secondary>
              {"Edit"->React.string}
            </Link.AsButton>
          </div>
          <div>
            {switch state {
            | Viewing({deletionError}) => <>
                <Button size=SM style=Danger onClick={_ => Delete->dispatch}>
                  {"Delete"->React.string}
                </Button>
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
      </Layout.HeaderWithControls>
      <Layout.Content>
        <div className=Css.content>
          <div className=Css.post> {post.content->React.string} </div>
          <Hr />
          <div className=Css.footer>
            <Link route={Route.posts}> {"Back to posts"->React.string} </Link>
          </div>
        </div>
      </Layout.Content>
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

  <Layout>
    {switch state {
    | Loading => "Loading..."->React.string
    | Ready(post) => <PostView post />
    | Failure => "Oh no"->React.string
    }}
  </Layout>
}
