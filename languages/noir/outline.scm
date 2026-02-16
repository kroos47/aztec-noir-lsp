; Document outline for Noir (tsujp/tree_sitter_noir grammar)

(function_item
  name: (identifier) @name) @item

(function_signature_item
  name: (identifier) @name) @item

(struct_item
  name: (identifier) @name) @item

(impl_item
  (identifier) @name) @item

(trait_item
  name: (identifier) @name) @item

(module_or_contract_item
  name: (identifier) @name) @item

(global_item
  name: (identifier) @name) @item

(type_item
  name: (identifier) @name) @item
