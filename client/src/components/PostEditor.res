module Css = %css(
  let form = css`
    display: grid;
    grid-auto-rows: max-content;
    grid-template-columns: 1fr;
    grid-row-gap: 30px;
    align-items: center;
    justify-items: end;
  `

  let row = css`
    display: grid;
    grid-template-columns: 1fr;
    align-items: center;
    justify-items: start;
    width: 100%;
  `

  let buttons = css`
    display: grid;
    grid-template-columns: max-content max-content;
    grid-column-gap: 24px;
    align-items: center;
    justify-content: end;
    width: 100%;
  `
)

type status = Editing({submissionStatus: option<result<unit, option<string>>>}) | Submitting

type input = {
  title: string,
  content: string,
}

type state = {status: status, input: input}

type action =
  | UpdateTitleInput(string)
  | UpdateContentInput(string)
  | Submit
  | FailSubmission({reason: option<string>})

type submit = (input, ~onFailure: (~reason: string=?, unit) => unit) => option<unit => unit>

@react.component
let make = (~initialInput, ~cancelRoute, ~onSubmit as submit: submit) => {
  let (state, dispatch) = ReactUpdate.useReducerWithMapState(
    () => {
      status: Editing({submissionStatus: None}),
      input: initialInput,
    },
    (action, state) =>
      switch action {
      | UpdateTitleInput(value) =>
        switch state.status {
        | Editing(_) => Update({...state, input: {...state.input, title: value}})
        | Submitting => NoUpdate
        }
      | UpdateContentInput(value) =>
        switch state.status {
        | Editing(_) => Update({...state, input: {...state.input, content: value}})
        | Submitting => NoUpdate
        }
      | Submit =>
        UpdateWithSideEffects(
          {...state, status: Submitting},
          ({state, send: dispatch}) => {
            state.input->submit(~onFailure=(~reason=?, ()) =>
              FailSubmission({reason: reason})->dispatch
            )
          },
        )
      | FailSubmission({reason}) =>
        Update({...state, status: Editing({submissionStatus: Some(Error(reason))})})
      },
  )

  <Form className=Css.form onSubmit={() => Submit->dispatch}>
    <div className=Css.row>
      <TextField
        id="title"
        placeholder="Title"
        value=state.input.title
        status=?{switch state.status {
        | Editing(_) => None
        | Submitting => Some(InsensiblyDisabled)
        }}
        onChange={event => UpdateTitleInput(ReactEvent.Form.target(event)["value"])->dispatch}
      />
    </div>
    <div className=Css.row>
      <TextArea
        id="content"
        placeholder="Content"
        rows=5
        value=state.input.content
        status=?{switch state.status {
        | Editing(_) => None
        | Submitting => Some(InsensiblyDisabled)
        }}
        onChange={event => UpdateContentInput(ReactEvent.Form.target(event)["value"])->dispatch}
      />
    </div>
    {switch state.status {
    | Editing({submissionStatus: None | Some(Ok())}) | Submitting => React.null
    | Editing({submissionStatus: Some(Error(reason))}) =>
      <div>
        {switch reason {
        | Some(reason) => reason->React.string
        | None => "Something went wrong"->React.string
        }}
      </div>
    }}
    <div className=Css.buttons>
      <Link route={cancelRoute}> {"Cancel"->React.string} </Link>
      <Button
        kind=#submit
        size=MD
        style=Primary
        status=?{switch state.status {
        | Submitting => Some(Busy({label: "Saving..."}))
        | Editing(_) => None
        }}>
        {"Save"->React.string}
      </Button>
    </div>
  </Form>
}
