module Css = %css(
  let textarea =
    css`
      display: flex;
      flex: 1;
      width: 100%;
      transition-property: border-color, box-shadow;
      transition-duration: ${Transition.fast};
      transition-timing-function: ${Transition.timingFunction};
      color: ${Color.text};
      background-color: transparent;
      font-variant: normal;
      font-stretch: normal;
      font-family: ${Font.mono};
      font-weight: ${Font.normal};
      font-style: normal;
      border: 1px solid transparent;
      border-bottom: 1px solid ${Form.fieldBorderColor};
      padding: ${Form.fieldPadding}px;
      font-size: ${Font.size}px;
      &::placeholder {
        color: ${Form.fieldPlaceholderColor};
      }
      &:focus {
        border-bottom: 1px solid ${Color.primary};
        outline: 0 none transparent;
      }
      &:disabled {
        cursor: not-allowed;
        background-color: transparent;
      }
    `

  let off =
    css`
      color: ${Form.fieldPlaceholderColor};
      background-color: ${Form.fieldDisabledBgColor};
    `
)

module Status = {
  type t =
    | InsensiblyDisabled
    | VisuallyDisabled

  let style = status =>
    switch status {
    | InsensiblyDisabled => Cn.none
    | VisuallyDisabled => Css.off
    }
}

@react.component
let make = (
  ~id: string,
  ~value: string,
  ~placeholder: option<string>=?,
  ~rows: int,
  ~status: option<Status.t>=?,
  ~onChange: ReactEvent.Form.t => unit,
) =>
  <textarea
    id
    value
    rows
    ?placeholder
    className={Css.textarea->Cn.append(status->Cn.mapSome(status =>
        switch status {
        | InsensiblyDisabled => Cn.none
        | VisuallyDisabled => Css.off
        }
      ))}
    disabled={switch status {
    | None => false
    | Some(InsensiblyDisabled)
    | Some(VisuallyDisabled) => true
    }}
    onChange
  />
