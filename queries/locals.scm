;; Scopes

[(module)
 (class_definition_scope)
 (function_definition_scope)
 (lambda)] @local.scope

;; Definitions

; TODO: Type parameters, patterns in assignment and match statements

(import_statement (dotted_name . _ @local.definition))
(import_from_statement name: (dotted_name . _ @local.definition))
(aliased_import alias: _ @local.definition)
(class_definition name: _ @local.definition)
(type_alias_statement left: (type . (identifier) @local.definition))
(function_definition name: _ @local.definition)
(parameter name: _ @local.definition)
(as_pattern alias: _ @local.definition)
(named_expression name: (identifier) @local.definition)
(assignment left: (identifier) @local.definition)

;; References

(identifier) @local.reference
