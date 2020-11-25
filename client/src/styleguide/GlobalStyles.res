include %css(
  let global =
    css`
      :global() {
        * {
          box-sizing: border-box;
          transform-origin: 50% 50% 0;
          margin: 0;
          padding: 0;
          outline-width: 0;
          outline-style: none;
          outline-offset: 0;
          -moz-osx-font-smoothing: grayscale;
          -webkit-font-smoothing: antialiased;
          text-rendering: optimizeLegibility;
          -webkit-tap-highlight-color: transparent;
        }

        html {
          font-family: ${Font.mono};
          font-size: ${Font.size}px;
          font-weight: ${Font.normal};
          line-height: ${Font.lineHeight};
        }
      }
    `
)
