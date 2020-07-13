use crate::vm::class::signature::FieldSig;


/// A field of a class.
/// Can be static (class field) or non-static (instance field).
#[derive(Debug, Clone)]
pub struct Field {
    signature: FieldSig,
    is_static: bool,
}


impl Field {
    /// Creates a new field with the given type descriptor and name.
    pub fn new(signature: FieldSig, is_static: bool) -> Self {
        Field {
            signature,
            is_static,
        }
    }

    /// Returns the field signature.
    pub fn signature(&self) -> &FieldSig {
        &self.signature
    }

    /// Returns true if the field is static, false otherwise.
    pub fn is_static(&self) -> bool {
        self.is_static
    }
}
