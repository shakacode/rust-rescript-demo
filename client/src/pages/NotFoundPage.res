@react.component
let make = () => {
  <div>
    <h1> {"404"->React.string} </h1>
    <div> <Router.Link route=Route.posts> {"Main"->React.string} </Router.Link> </div>
  </div>
}
