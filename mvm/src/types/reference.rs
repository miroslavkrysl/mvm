use std::fmt;
use std::intrinsics::transmute;
use std::sync::Arc;
use crate::class::instance::Instance;
use crate::types::error::ValueError;


#[derive(Debug, Clone)]
pub enum Reference {
    Null,
    Instance(Arc<Instance>),
}


impl Reference {
    pub fn null() -> Self {
        Reference::Null
    }

    pub fn new(ptr: Arc<Instance>) -> Self {
        Reference::Instance(ptr)
    }

    pub fn is_null(&self) -> bool { 
        match self {
            Reference::Null => true,
            Reference::Instance(_) => false,
        }
    }

    pub fn instance(self) -> Result<Arc<Instance>, ValueError> {
        match self {
            Reference::Null => Err(ValueError::NullPointer),
            Reference::Instance(instance) => Ok(instance),
        }
    }
}


impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Reference::Null, Reference::Null) => true,
            (Reference::Instance(ptr1), Reference::Instance(ptr2)) => Arc::ptr_eq(ptr1, ptr2),
            _ => false
        }
    }
}


impl Eq for Reference {}


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