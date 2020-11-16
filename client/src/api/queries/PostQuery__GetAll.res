open PostFragment

module Query = %graphql(
  `
    query GetAllPosts {
      posts {
        ...PostFragment
      }
    }
  `
)
