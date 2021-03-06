module type Interface = {
  type t

  external make: string => t = "%identity"
  external toString: t => string = "%identity"

  let parse: Js.Json.t => t
  let serialize: t => Js.Json.t
}

module Make = (): Interface => {
  type t = string

  external make: string => t = "%identity"
  external toString: t => string = "%identity"

  let parse = json => json->Js.Json.decodeString->Option.getUnsafe
  let serialize = id => id->Js.Json.string
}
