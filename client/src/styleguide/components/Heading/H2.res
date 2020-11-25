module Css = HeadingStyles

@react.component
let make = (~children) => {
  <h2 className={Css.h->Cn.append(Css.h2)}> {children} </h2>
}
