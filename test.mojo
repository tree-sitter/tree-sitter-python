alias aaa = __mlir_type[`!kgen.pointer<!kgen.pointer<scalar<ui8>>>`]
alias f = __mlir_type[`!kgen.signature<`, Int, `>() -> !kgen.none`]
alias aa = __mlir_type.`!kgen.array`
alias a = __mlir_type.`i1`
alias int16 = DType(__mlir_attr.`#kgen.dtype.constant<si16> : !kgen.dtype`)
alias _mlir_type = __mlir_type[
    `!lit.ref.pack<:variadic<`,
    element_trait,
    `> `,
    element_types,
    `, `,
    lifetime,
    `, `,
    +address_space,
    `>`,
]

var file_name: __mlir_type.`!kgen.string`
line, col, file_name = __mlir_op.`kgen.source_loc`[
    _properties = __mlir_attr.`{inlineCount = 1 : i64}`,
    _type = (
        __mlir_type.index,
        __mlir_type.index,
        __mlir_type.`!kgen.string`,
    ),
]()

# return __mlir_op.`index.casts`[_type = __mlir_type.index](self.value)
