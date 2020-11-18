@react.component
let make = () => {
  <div>
    <div> <input type_="text" placeholder="Title" value="" onChange=ignore /> </div>
    <div> <textarea value="" onChange=ignore /> </div>
    <div> <Router.Link route={Route.posts}> {"Save"->React.string} </Router.Link> </div>
  </div>
}
