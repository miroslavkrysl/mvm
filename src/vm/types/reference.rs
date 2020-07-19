use std::fmt;
use std::intrinsics::transmute;
use crate::vm::class::instance::Instance;
use crate::vm::types::error::ValueError;


#[derive(Debug, Clone)]
pub enum Reference {
    Null,
    Instance(Instance),
}


impl Reference {
    pub fn null() -> Self {
        Reference::Null
    }

    pub fn new(ptr: Instance) -> Self {
        Reference::Instance(ptr)
    }

    pub fn is_null(&self) -> bool {
        match self {
            Reference::Null => true,
            Reference::Instance(_) => false,
        }
    }

    pub fn to_instance(self) -> Result<Instance, ValueError> {
        match self {
            Reference::Null => Err(ValueError::NullPointer),
            Reference::Instance(instance) => Ok(instance),
        }
    }

    pub fn as_instance(&self) -> Result<&Instance, ValueError> {
        match self {
            Reference::Null => Err(ValueError::NullPointer),
            Reference::Instance(instance) => Ok(instance),
        }
    }

    pub fn eq(&self, other: &Reference) -> bool {
        match (self, other) {
            (Reference::Null, Reference::Null) => true,
            (Reference::Instance(ptr1), Reference::Instance(ptr2)) => ptr1.id() == ptr2.id(),
            _ => false
        }
    }
}


impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reference::Null => write!(f, "null"),
            Reference::Instance(instance) => {
                let ptr = unsafe { transmute::<_, usize>(instance) };
                write!(f, "{}@{:x}", instance.class().name(), ptr)
            }
        }
    }
}


impl Default for Reference {
    fn default() -> Self {
        Self::null()
    }
}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Reference::Null, Reference::Null) => true,
            (Reference::Instance(ptr1), Reference::Instance(ptr2)) => ptr1.id() == ptr2.id(),
            _ => false
        }
    }
}