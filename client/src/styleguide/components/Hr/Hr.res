module Css = %css(
  let hr =
    css`
      display: flex;
      width: 100%;
      border-width: 0;
      border-top: 1px solid ${Color.lightGrayLine};
    `
)

@react.component
let make = () => {
  <hr className={Css.hr} />
}
