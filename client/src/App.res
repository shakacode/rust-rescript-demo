include GlobalStyles

@react.component
let make = () => {
  let route = Router.useRouter()

  switch route {
  | Some(Posts) => <PostsPage />
  | Some(Post({id})) => <PostPage id />
  | Some(NewPost) => <NewPostPage />
  | Some(EditPost({id})) => <EditPostPage id />
  | None => <NotFoundPage />
  }
}
