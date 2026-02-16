; ============================================================
; Noir highlights.scm for Zed editor
; Based on tree-sitter-noir grammar (tsujp/tree_sitter_noir)
; ============================================================

; ---------- Keywords ----------
"fn" @keyword
"let" @keyword
"pub" @keyword
"struct" @keyword
"impl" @keyword
"mod" @keyword
"contract" @keyword
"use" @keyword
"as" @keyword
"for" @keyword
"in" @keyword
"if" @keyword
"else" @keyword
"type" @keyword
"global" @keyword
"comptime" @keyword
"trait" @keyword
"where" @keyword
"return" @keyword
"break" @keyword
"continue" @keyword
"unsafe" @keyword
"unconstrained" @keyword

(mutable_modifier) @keyword

; ---------- Built-in values ----------
(bool_literal) @constant
(self) @variable.special
(super) @keyword
(crate) @keyword
(dep) @keyword

; ---------- Functions ----------
; Function definitions
(function_item
  name: (identifier) @function)

(function_signature_item
  name: (identifier) @function)

; Function calls
(call_expression
  function: (identifier) @function)

; Method calls (via access expression as call target)
(call_expression
  function: (access_expression
    name: (identifier) @function.method))

; ---------- Types ----------
(primitive_type) @type.builtin

(struct_item
  name: (identifier) @type)

(trait_item
  name: (identifier) @type)

(type_item
  name: (identifier) @type)

(impl_item
  (identifier) @type)

(type_arguments) @type

; ---------- Variables & Parameters ----------
(let_statement
  pattern: (identifier) @variable)

(parameter
  (identifier) @variable.parameter)

; Self parameter
(self_pattern) @variable.special

; Struct field definitions
(struct_field_item
  name: (identifier) @property)

; Struct field initializers
(field_initializer
  (identifier) @property)

; Struct patterns
(struct_pattern_field
  (identifier) @property)

; ---------- Constants (ALL_CAPS convention) ----------
((identifier) @constant
  (#match? @constant "^[A-Z][A-Z\\d_]+$"))

; Global items
(global_item
  name: (identifier) @constant)

; ---------- Modules ----------
(module_or_contract_item
  name: (identifier) @title)

; Use/Import paths
(use_item
  (path) @title)

; ---------- Literals ----------
(str_literal) @string
(raw_str_literal) @string
(fmt_str_literal) @string
(escape_sequence) @string.escape
(int_literal) @number

; ---------- Comments ----------
(line_comment) @comment
(block_comment) @comment
(doc_comment) @comment

; ---------- Visibility ----------
(visibility_modifier) @keyword

; ---------- Attributes (Aztec decorators like #[aztec(...)]) ----------
(attribute_item) @attribute

; ---------- Operators ----------
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"=" @operator
"==" @operator
"!=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"!" @operator
"&" @operator
"|" @operator
"^" @operator
"<<" @operator
">>" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"%=" @operator
"&=" @operator
"|=" @operator
"^=" @operator
"<<=" @operator
">>=" @operator
"->" @operator

; ---------- Punctuation ----------
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

"::" @punctuation.delimiter
":" @punctuation.delimiter
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter

; ---------- Assert builtins ----------
(constrain_statement
  "assert" @function.builtin)
(constrain_statement
  "assert_eq" @function.builtin)
