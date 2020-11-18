type t =
  | Posts
  | Post({id: PostId.t})
  | NewPost
  | EditPost({id: PostId.t})

let fromUrl = (url: ReasonReactRouter.url) =>
  switch url.path {
  | list{} => Posts->Some
  | list{"posts", "new"} => NewPost->Some
  | list{"posts", id} => Post({id: id->PostId.make})->Some
  | list{"posts", id, "edit"} => EditPost({id: id->PostId.make})->Some
  | _ => None
  }

type t'

external make: string => t' = "%identity"
external toString: t' => string = "%identity"

let posts = "/"->make
let post = (~id: PostId.t) => (`/posts/${id->PostId.toString}`)->make
let newPost = "/posts/new"->make
let editPost = (~id: PostId.t) => (`/posts/${id->PostId.toString}/edit`)->make
