%graphql(
  `
    fragment PostFragment on Post @ppxAs(type: "Post.t") {
      id @ppxCustom(module: "PostId")
      title
      content
    }
  `
)
