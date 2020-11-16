module Query = %graphql(
  `
    mutation DeletePost($id: PostId!) {
      result: deletePost(id: $id) {
        ok
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
