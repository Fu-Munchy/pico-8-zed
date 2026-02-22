;; This code was taken from
;; https://github.com/zed-extensions/lua/blob/main/languages/lua/textobjects.scm
;; without any alterations.
(function_definition
  body: (_) @function.inside) @function.around

(function_declaration
  body: (_) @function.inside) @function.around

(comment)+ @comment.around
