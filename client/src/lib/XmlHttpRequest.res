type t

@new external make: unit => t = "XMLHttpRequest"

@get external response: t => string = "response"

@get external status: t => int = "status"

type readyState = UNSENT | OPENED | HEADERS_RECEIVED | LOADING | DONE
@get external readyState: t => int = "readyState"
let readyState = x =>
  switch x->readyState {
  | 0 => UNSENT
  | 1 => OPENED
  | 2 => HEADERS_RECEIVED
  | 3 => LOADING
  | 4 => DONE
  | _ => failwith("Unexpected readyState")
  }

@send external openAsync: (t, string, string, @bs.as(json`true`) _) => unit = "open"
@send external setRequestHeader: (t, string, string) => unit = "setRequestHeader"
@send external send: (t, string) => unit = "send"
@send external abort: t => unit = "abort"

@send
external on: (t, @bs.string [#readystatechange(unit => unit)]) => unit = "addEventListener"
