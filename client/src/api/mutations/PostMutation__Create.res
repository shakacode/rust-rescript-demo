open PostFragment

module Query = %graphql(
  `
    mutation CreatePost($input: CreatePostInput!) {
      post: createPost(input: $input) {
        ...PostFragment
      }
    }
  `
)

module Variables = {
  let make = (~title, ~content) => {Query.input: {title: title, content: content}}
}
