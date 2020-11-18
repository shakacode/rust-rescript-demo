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

  let submitting = React.useMemo1(() => {
    switch state.status {
    | Submitting => true
    | Editing(_) => false
    }
  }, [state.status])

  <form
    onSubmit={event => {
      event->ReactEvent.Form.preventDefault
      Submit->dispatch
    }}>
    <div>
      <input
        type_="text"
        placeholder="Title"
        disabled=submitting
        value=state.input.title
        onChange={event => UpdateTitleInput(ReactEvent.Form.target(event)["value"])->dispatch}
      />
    </div>
    <div>
      <textarea
        disabled=submitting
        value=state.input.content
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
    <div>
      <button type_="submit" disabled=submitting>
        {(submitting ? "Saving..." : "Save")->React.string}
      </button>
      <Router.Link route={cancelRoute}> {"Cancel"->React.string} </Router.Link>
    </div>
  </form>
}
