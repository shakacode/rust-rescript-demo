open PostFragment

module Query = %graphql(
  `
    mutation UpdatePost($input: UpdatePostInput!) {
      post: updatePost(input: $input) {
        ...PostFragment
      }
    }
  `
)

module Variables = {
  let make = (~id, ~title, ~content) => {Query.input: {id: id, title: title, content: content}}
}

module ExtendedError = {
  type t = PostNotFound

  let parse = (~reason, ~payload as _) => {
    switch reason {
    | "POST_NOT_FOUND" => Ok(PostNotFound)
    | _ as reason => Error(`Unexpected reason: ${reason}`)
    }
  }
}
