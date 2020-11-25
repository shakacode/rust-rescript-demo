module Css = HeadingStyles

@react.component
let make = (~className="", ~children) => {
  <h1 className={Css.h->Cn.append(Css.h1)->Cn.append(className)}> {children} </h1>
}
