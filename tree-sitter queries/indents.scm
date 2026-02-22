;; This code was taken from
;; https://github.com/zed-extensions/lua/blob/main/languages/lua/indents.scm
;; without any alterations.
(if_statement "end" @end) @indent
(do_statement "end" @end) @indent
(while_statement "end" @end) @indent
(for_statement "end" @end) @indent
(repeat_statement "until" @end) @indent
(function_declaration "end" @end) @indent
(function_definition "end" @end) @indent

(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent
