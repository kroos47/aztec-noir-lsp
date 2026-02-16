; Indentation rules for Noir (tsujp/tree_sitter_noir grammar)

(function_item (block) @indent)
(function_signature_item) @indent
(struct_item (struct_field_list) @indent)
(impl_item) @indent
(trait_item) @indent
(for_statement (block) @indent)
(if_expression (block) @indent)
(module_or_contract_item (item_list) @indent)
(lambda) @indent
(unsafe (block) @indent)
(comptime (block) @indent)
