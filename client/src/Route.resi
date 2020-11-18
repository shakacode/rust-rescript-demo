type t =
  | Posts
  | Post({id: PostId.t})
  | NewPost
  | EditPost({id: PostId.t})

let fromUrl: ReasonReactRouter.url => option<t>

type t'

external toString: t' => string = "%identity"

let posts: t'
let post: (~id: PostId.t) => t'
let newPost: t'
let editPost: (~id: PostId.t) => t'
