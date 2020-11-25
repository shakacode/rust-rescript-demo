module Css = %css(
  let width = 600

  let global =
    css`
      :global() {
        html {
          min-height: 100%;
        }

        html,
        body,
        #app {
          display: grid;
          position: relative;
          grid-template-columns: ${width}px;
          grid-template-rows: 1fr;
          justify-content: center;
          margin: 0;
          padding: 0;

          @media ${Mq.smallScreen} {
            grid-template-columns: 100%;
            padding: 0 14px;
          }
        }
      }
    `

  let container = css`
    display: grid;
    grid-template-rows: max-content 1fr max-content;
    grid-template-columns: 1fr;
    grid-row-gap: 10px;
    margin: 10px 0;
  `

  let header = css`
    display: grid;
    grid-auto-columns: max-content;
  `

  let headerWithControls = css`
    display: grid;
    grid-template-columns: 1fr max-content;
    align-items: center;
  `

  let content = css`
    display: grid;
    grid-auto-rows: max-content;
  `

  let footer = css`
    display: grid;
  `
)

module Header = {
  @react.component
  let make = (~children) => {
    <div className=Css.header> {children} </div>
  }
}

module HeaderWithControls = {
  @react.component
  let make = (~children) => {
    <div className=Css.headerWithControls> {children} </div>
  }
}

module Content = {
  @react.component
  let make = (~children) => {
    <div className=Css.content> {children} </div>
  }
}

module Footer = {
  @react.component
  let make = () => {
    <div className=Css.footer> {"(c)"->React.string} </div>
  }
}

@react.component
let make = (~children) => {
  <div className=Css.container> {children} <Footer /> </div>
}
