/// Struct to store the method information
pub(crate) struct MethodOld {
    pub(crate) pallet_name: String,
    pub(crate) method_name: String,
    pub(crate) arguments: Vec<Argument>,
    pub(crate) docs: String,
}

/// Struct to store the argument name and type
pub(crate) struct Argument {
    pub(crate) name: String,
    pub(crate) ty: String,
}

/// Struct to store current method and remaining data
pub(crate) struct NextDecodeOld {
    pub(crate) method: MethodOld,
    pub(crate) data: Vec<u8>,
}
