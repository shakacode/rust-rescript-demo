let initialInput = {PostEditor.title: "", content: ""}

@react.component
let make = () => {
  <Layout>
    <Layout.Header> <H1> {"Add post"->React.string} </H1> </Layout.Header>
    <Layout.Content>
      <PostEditor
        initialInput
        cancelRoute=Route.posts
        onSubmit={(input, ~onFailure as fail) => {
          open PostMutation__Create
          Api.exec(
            ~query=module(Query),
            ~variables=Variables.make(~title=input.title, ~content=input.content),
            ~extendedError=None,
            res =>
              switch res {
              | Ok(res) => Route.post(~id=res.post.id)->Router.push
              | Error(_) => fail()
              },
          )
        }}
      />
    </Layout.Content>
  </Layout>
}
