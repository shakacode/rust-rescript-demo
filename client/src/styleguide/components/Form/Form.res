let fieldPadding = 8
let fieldBorderColor = Color.gray
let fieldPlaceholderColor = Color.lightGray
let fieldDisabledBgColor = Color.lightGrayBg

@react.component
let make = (~className=?, ~onSubmit as submit, ~children) =>
  <form
    ?className
    onSubmit={event => {
      event->ReactEvent.Form.preventDefault
      submit()
    }}>
    children
  </form>
