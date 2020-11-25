module Css = %css(
  let button =
    css`
      display: flex;
      position: relative;
      align-items: center;
      justify-content: center;
      transition-property: box-shadow, background-color;
      transition-duration: ${Transition.fast};
      transition-timing-function: ${Transition.timingFunction};
      border-width: 1px;
      border-style: solid;
      border-color: transparent;
      cursor: default;
      font-family: ${Font.mono};
      text-align: center;
      text-decoration: none;
      line-height: normal;
      user-select: none;
      white-space: nowrap;
      outline: 0 none transparent;

      &:active {
        transform: translateY(1px);
        background-image: none;
      }

      &:focus:not(:disabled) svg:first-child,
      &:hover:not(:disabled) svg:first-child {
        transform: rotate(-7deg) scale(1.2);
        transition-property: transform;
        transition-duration: 0.12s;
        transition-timing-function: ${Transition.timingFunction};
      }

      &:disabled {
        cursor: not-allowed;
        transform: none;
      }
    `

  let inner = css`
    display: inline-flex;
    flex-flow: row nowrap;
    align-items: center;
  `

  let sm = css`
    border-radius: 2px;
    padding: 6px 10px;
    font-size: 12px;
  `

  let md = css`
    border-radius: 3px;
    padding: 8px 16px;
    font-size: ${Font.size}px;
  `

  let lg = css`
    border-radius: 4px;
    padding: 12px 24px;
    font-size: 20px;
  `

  let primary =
    css`
      color: ${Color.white};
      background-color: ${Color.primary};

      &:focus,
      &:hover:not(:disabled) {
        background-color: ${Color.primary->Polished.lighten(
      0.05,
    )};
      }

      &:focus {
        box-shadow: 0 0 0 4px ${Color.primary->Polished.tint(
      0.6,
    )};
      }

      &.${off} {
        color: ${Color.white};
        background-color: ${Color.primary->Polished.lighten(
      0.25,
    )};
      }
    `

  let secondary =
    css`
      color: ${Color.darkGray};
      border-color: ${Color.gray};

      background-color: transparent;

      &:focus,
      &:hover:not(:disabled) {
        color: ${Color.darkGray};
        border-color: ${Color.gray};
      }

      &:focus {
        box-shadow: 0 0 0 4px ${Color.darkGray->Polished.tint(
      0.6,
    )};
      }

      &.${off} {
        color: ${Color.gray->Polished.lighten(
      0.25,
    )};
        border-color: ${Color.gray->Polished.lighten(0.25)};
      }
    `

  let danger =
    css`
      color: ${Color.white};
      background-color: ${Color.danger};

      &:focus,
      &:hover:not(:disabled) {
        background-color: ${Color.danger->Polished.lighten(
      0.05,
    )};
      }

      &:focus {
        box-shadow: 0 0 0 4px ${Color.danger->Polished.tint(
      0.6,
    )};
      }

      &.${off} {
        color: ${Color.white};
        background-color: ${Color.danger->Polished.lighten(
      0.25,
    )};
      }
    `

  let busy = css`
    pointer-events: none;
  `

  let off = css`
    cursor: not-allowed;
    transform: none;
  `
)

module Size = {
  type t =
    | SM
    | MD
    | LG

  let style = size =>
    switch size {
    | SM => Css.sm
    | MD => Css.md
    | LG => Css.lg
    }
}

module Style = {
  type t =
    | Primary
    | Secondary
    | Danger

  let style = style =>
    switch style {
    | Primary => Css.primary
    | Secondary => Css.secondary
    | Danger => Css.danger
    }
}

module Status = {
  type t =
    | InsensiblyDisabled
    | VisuallyDisabled
    | Busy({label: string})

  let style = status =>
    switch status {
    | InsensiblyDisabled => Cn.none
    | VisuallyDisabled => Css.off
    | Busy(_) => Css.busy
    }
}

module Kind = {
  type t = [#button | #submit]

  external toString: t => string = "%identity"
}

@react.component
let make = (
  ~id: option<string>=?,
  ~size: Size.t,
  ~style: Style.t,
  ~kind=#button,
  ~status: option<Status.t>=?,
  ~onClick=?,
  ~children,
) =>
  <button
    ?id
    type_={kind->Kind.toString}
    disabled={switch status {
    | None => false
    | Some(InsensiblyDisabled | VisuallyDisabled | Busy(_)) => true
    }}
    className={Css.button
    ->Cn.append(size->Size.style)
    ->Cn.append(style->Style.style)
    ->Cn.append(status->Cn.mapSome(Status.style))}
    ?onClick>
    {switch status {
    | None
    | Some(InsensiblyDisabled) =>
      <span className={Css.inner}> children </span>
    | Some(VisuallyDisabled) => <span className={Css.inner->Cn.append(Css.off)}> children </span>
    | Some(Busy({label})) =>
      <span className={Css.inner->Cn.append(Css.busy)}> {label->React.string} </span>
    }}
  </button>
