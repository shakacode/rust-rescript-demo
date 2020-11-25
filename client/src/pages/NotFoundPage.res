module Css = %css(
  let container = css`
    display: grid;
    position: relative;
    grid-template-columns: max-content max-content max-content;
    grid-template-rows: 1fr;
    grid-column-gap: 25px;
    align-items: center;
    justify-content: center;
  `

  let line =
    css`
      display: flex;
      border-width: 0;
      border-right: 1px solid ${Color.lightGrayLine};
      height: 70px;
    `
)

@react.component
let make = () => {
  <div className=Css.container>
    <H1> {"404"->React.string} </H1>
    <div className=Css.line />
    <Link route=Route.posts> {"Main"->React.string} </Link>
  </div>
}
