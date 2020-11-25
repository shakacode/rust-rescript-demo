module Css = %css(
  let link =
    css`
      display: inline;
      position: relative;
      margin: -1px;
      padding: 1px;
      background-color: transparent;
      cursor: pointer;
      border-radius: 1px;

      color: ${Color.text};

      text-decoration: underline;
      text-decoration-skip: ink;
      text-decoration-skip-ink: auto;
      text-decoration-style: solid;

      -webkit-text-decoration-skip: objects;

      &:focus {
        box-shadow: 0 0 0 4px ${Color.outline};
      }
    `
)

@react.component
let make = (~route: Route.t', ~children) => {
  <Router.Link route className={Css.link}> children </Router.Link>
}

module AsButton = {
  @react.component
  let make = (~route: Route.t', ~size: Button.Size.t, ~style: Button.Style.t, ~children) => {
    <Router.Link
      route
      className={Button.Css.button
      ->Cn.append(size->Button.Size.style)
      ->Cn.append(style->Button.Style.style)}>
      children
    </Router.Link>
  }
}
