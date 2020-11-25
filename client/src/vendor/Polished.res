@module("polished") external lighten: (float, string) => string = "lighten"
let lighten = (color, amount) => lighten(amount, color)

@module("polished") external tint: (float, string) => string = "tint"
let tint = (color, amount) => tint(amount, color)
