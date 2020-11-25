module Css = %css(
  let posts = css`
    display: grid;
    grid-auto-rows: max-content;
    grid-row-gap: 18px;
  `

  let post = css`
    display: grid;
    grid-template-rows: max-content max-content;
    grid-row-gap: 5px;
  `

  let postContent = css`
    display: flex;
    flex-flow: row nowrap;
  `
)

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

  <Layout>
    <Layout.HeaderWithControls>
      <H1> {"Posts"->React.string} </H1>
      <div>
        <Link.AsButton route={Route.newPost} size=SM style=Secondary>
          {"Add Post"->React.string}
        </Link.AsButton>
      </div>
    </Layout.HeaderWithControls>
    {switch state {
    | Loading => "Loading..."->React.string
    | Ready(posts) =>
      <Layout.Content>
        <div className=Css.posts>
          {posts
          ->Array.map(post =>
            <div key={post.id->PostId.toString} className=Css.post>
              <H2> <Link route={Route.post(~id=post.id)}> {post.title->React.string} </Link> </H2>
              <div className=Css.postContent> {post.content->React.string} </div>
            </div>
          )
          ->React.array}
        </div>
      </Layout.Content>
    | Failure => "Oh no"->React.string
    }}
  </Layout>
}
