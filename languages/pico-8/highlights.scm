; ;; The following file has been taken and modified from
; ;; https://github.com/tree-sitter-grammars/tree-sitter-lua/blob/main/queries/highlights.scm

; Keywords
[
  "goto"
  "in"
  "local"
  "global"
  "return"
] @keyword

(label_statement) @label

(break_statement) @keyword

(do_statement
  [
    "do"
    "end"
  ] @keyword)

(while_statement
  [
    "while"
    "do"
    "end"
  ] @keyword)

(repeat_statement
  [
    "repeat"
    "until"
  ] @keyword)

(if_statement
  [
    "if"
    "elseif"
    "else"
    "then"
    "end"
  ] @keyword)

(elseif_statement
  [
    "elseif"
    "then"
    "end"
  ] @keyword)

(else_statement
  [
    "else"
    "end"
  ] @keyword)

(for_statement
  [
    "for"
    "do"
    "end"
  ] @keyword)

(function_declaration
  [
    "function"
    "end"
  ] @keyword)

; Operators
(binary_expression
  operator: _ @operator)

(unary_expression
  operator: _ @operator)

[
  "+"
  "-"
  "*"
  "/"
  "%"
  "^"
  ; These operators, although found in pico-8, are not part of the Lua tree-sitter grammar.
  ; List from https://iiviigames.github.io/pico8-api/
  ; "#"
  ; "=="
  ; "~="
  ; "<="
  ; ">="
  ; "<"
  ; ">"
  ; "="
  ; "&"
  ; "~"
  ; "|"
  ; "<<"
  ; ">>"
  ; "|"
  ; "^^"
  ; "<<>"
  ; ">><"
  ; "!="
  ; "%="
  ; "^="
  ; "/="
] @operator

[
  "and"
  "not"
  "or"
] @keyword.operator

; Punctuations
[
  ";"
  ":"
  ","
  "."
] @punctuation.delimiter

; Brackets
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

; Variables
(identifier) @variable

((identifier) @variable.special
  (#eq? @variable.special "self"))

(variable_list
  (attribute
    "<" @punctuation.bracket
    (identifier) @attribute
    ">" @punctuation.bracket))

; Constants
((identifier) @constant
  (#match? @constant "^[A-Z][A-Z_0-9]*$"))

(vararg_expression) @constant

(nil) @constant.builtin

[
  (false)
  (true)
] @boolean

; Tables
(field
  name: (identifier) @field)

(dot_index_expression
  field: (identifier) @field)

(table_constructor
  [
    "{"
    "}"
  ] @constructor)

; Functions
(parameters
  (identifier) @parameter)

(function_declaration
  name: [
    (identifier) @function
    (dot_index_expression
      field: (identifier) @function)
  ])

(function_declaration
  name: (method_index_expression
    method: (identifier) @method))

(assignment_statement
  (variable_list
    .
    name: [
      (identifier) @function
      (dot_index_expression
        field: (identifier) @function)
    ])
  (expression_list
    .
    value: (function_definition)))

(table_constructor
  (field
    name: (identifier) @function
    value: (function_definition)))

(function_call
  name: [
    (identifier) @function.call
    (dot_index_expression
      field: (identifier) @function.call)
    (method_index_expression
      method: (identifier) @method.call)
  ])

(function_call
  (identifier) @function
  (#any-of? @function
    ; built-in functions in Lua 5.1
    "rawequal" "rawget" "rawset" "select" "rawlen"

    ;; Pico-8 built-in functions, list from https://iiviigames.github.io/pico8-api/
    "spr" "music" "sfx"
    "cartdata" "dget" "dset" "cocreate" "coresume" "costatus" "yield" "camera"
    "circ" "circfill" "clip" "cls" "color" "cursor" "fget" "fillp" "flip" "fset"
    "line" "oval" "ovalfill" "pal" "palt" "pget" "print" "pset" "rect" "rectfill"
    "sget" "spr" "sset" "sspr" "tline" "btn" "btnp" "poke" "map" "mget" "mset" "mapdraw"
    "abs" "atan2" "cos" "flr" "-flr" "max" "mid" "min" "rnd" "sgn" "sin" "sqrt" "srand"
    "cstore" "reload" "memcpy" "memset" "peek" "serial" "stat" "chr" "add" "all" "count"
    "del" "foreach" "ipairs" "pairs" "pack" "unpack" "setmemtable" "band" "bor" "bnot" "shl"
    "shr" "lshr" "rotl" "rotr"))

; Others
(comment) @comment

(hash_bang_line) @preproc

(number) @number

(string) @string

(escape_sequence) @string.escape
