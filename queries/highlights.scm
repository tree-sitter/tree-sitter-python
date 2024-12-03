; adapted from Zed's Python Config
; https://github.com/zed-industries/zed/blob/6657e301cd0ee9e7b7b5352957ef30728ae2a874/crates/languages/src/python/highlights.scm
(attribute attribute: (identifier) @property)
(type (identifier) @type)


; Function calls

(decorator) @function

(call
  function: (attribute attribute: (identifier) @function.method))
(call
  function: (identifier) @function)

; Function definitions

(function_definition
  name: (identifier) @function)

; Identifier naming conventions

((identifier) @type
 (#match? @type "^[A-Z]"))

((identifier) @constant
 (#match? @constant "^_*[A-Z][A-Z\\d_]*$"))

; Builtin functions

((call
  function: (identifier) @function.builtin)
 (#match?
   @function.builtin
   "^(abs|all|always_inline|any|ascii|bin|bool|breakpoint|bytearray|bytes|callable|chr|classmethod|compile|complex|constrained|delattr|dict|dir|divmod|enumerate|eval|exec|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|isinstance|issubclass|iter|len|list|locals|map|max|memoryview|min|next|object|oct|open|ord|pow|print|property|range|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|unroll|vars|zip|__mlir_attr|__mlir_op|__mlir_type|__import__)$"))

; Literals

[
  (none)
  (true)
  (false)
] @constant.builtin

[
  (integer)
  (float)
] @number

(comment) @comment
(string) @string
(escape_sequence) @escape

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

(interpolation
  "{" @punctuation.special
  "}" @punctuation.special) @embedded

; Docstrings.
(function_definition
  "async"?
  "def"
  name: (_)
  (parameters)?
  body: (block (expression_statement (string) @string.doc)))

[
  "-"
  "-="
  "!="
  "*"
  "**"
  "**="
  "*="
  "/"
  "//"
  "//="
  "/="
  "&"
  "%"
  "%="
  "^"
  "+"
  "->"
  "+="
  "<"
  "<<"
  "<="
  "<>"
  "="
  ":="
  "=="
  ">"
  ">="
  ">>"
  "|"
  "~"
  "and"
  "in"
  "is"
  "not"
  "or"
  "is not"
  "not in"
  "!"
] @operator

[
  "as"
  "alias"
  "assert"
  "async"
  "await"
  "borrowed"
  "break"
  "capturing"
  "class"
  "continue"
  "def"
  "del"
  "elif"
  "else"
  "escaping"
  "except"
  "exec"
  "finally"
  "fn"
  "for"
  "from"
  "global"
  "if"
  "import"
  "inout"
  "lambda"
  "mut"
  "nonlocal"
  "owned"
  "pass"
  "print"
  "raise"
  "raises"
  "read"
  "ref"
  "return"
  "struct"
  "trait"
  "try"
  "var"
  "while"
  "with"
  "yield"
  "match"
  "case"
] @keyword

(mlir_type "!" @punctuation.special (#set! "priority" 110))
(mlir_type ">" @punctuation.special (#set! "priority" 110))
(mlir_type "<" @punctuation.special (#set! "priority" 110))
(mlir_type "->" @punctuation.special (#set! "priority" 110))
(mlir_type "(" @punctuation.special (#set! "priority" 110))
(mlir_type ")" @punctuation.special (#set! "priority" 110))
(mlir_type "." @punctuation.special (#set! "priority" 110))
(mlir_type ":" @punctuation.special (#set! "priority" 110))
(mlir_type "+" @punctuation.special (#set! "priority" 110))
(mlir_type "-" @punctuation.special (#set! "priority" 110))
(mlir_type "*" @punctuation.special (#set! "priority" 110))
(mlir_type "," @punctuation (#set! "priority" 110))
