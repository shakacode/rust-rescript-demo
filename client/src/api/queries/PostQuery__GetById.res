open PostFragment

module Query = %graphql(
  `
    query GetPostById($id: PostId!) {
      post(id: $id) {
        ...PostFragment
      }
    }
  `
)

module Variables = {
  let make = (~id) => {Query.id: id->PostId.serialize}
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
