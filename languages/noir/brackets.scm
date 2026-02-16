; Noir brackets.scm for Zed editor
; Bracket matching / rainbow brackets
; Based on tsujp/tree_sitter_noir grammar

("(" @open ")" @close)
("[" @open "]" @close)
("{" @open "}" @close)

; Generic angle brackets
(type_arguments "<" @open ">" @close)
(type_parameters "<" @open ">" @close)
