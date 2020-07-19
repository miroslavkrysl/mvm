use std::collections::HashSet;
use std::rc::Rc;

use crate::vm::class::error::ClassError;
use crate::vm::class::field::Field;
use crate::vm::class::instance::Instance;
use crate::vm::class::method::Method;
use crate::vm::class::name::ClassName;
use crate::vm::class::signature::{FieldSig, MethodSig};
use crate::vm::types::value::Value;
use std::sync::{RwLock, Arc};


/// A class.
#[derive(Debug)]
pub struct Class {
    name: ClassName,
    fields: Vec<FieldEntry>,
    methods: Vec<Arc<Method>>,
    static_fields_values: RwLock<Vec<Value>>,
    nonstatic_fields_len: usize,
}


impl Class {
    /// Create a new class with the given name, fields and methods.
    ///
    /// # Errors
    ///
    /// Returns `ClassError::DuplicateField` if there are two fields with the same signature
    /// or `ClassError::DuplicateMethod` if there are two methods with the same signature.
    pub fn new<F, M>(name: ClassName, fields: F, methods: M) -> Result<Self, ClassError>
                     where F: IntoIterator<Item=Field>,
                           M: IntoIterator<Item=Method> {
        let f = fields.into_iter();
        let m = methods.into_iter();

        // count indexes of static and nonstatic fields and check for duplicates
        let mut fields = Vec::new();
        let mut static_fields_len = 0;
        let mut nonstatic_fields_len = 0;

        let mut present_fields: HashSet<FieldSig> = HashSet::with_capacity(f.size_hint().0);

        for field in f {
            if present_fields.contains(field.signature()) {
                return Err(ClassError::DuplicateField(field.signature().clone()));
            }

            present_fields.insert(field.signature().clone());

            if field.is_static() {
                fields.push(FieldEntry {
                    offset: static_fields_len,
                    field: Arc::new(field),
                });
                static_fields_len += 1;
            } else {
                fields.push(FieldEntry {
                    offset: nonstatic_fields_len,
                    field: Arc::new(field),
                });
                nonstatic_fields_len += 1;
            }
        }

        // initialize static fields values
        let static_fields_values = fields.iter()
                                         .filter(|entry| entry.field.is_static())
                                         .map(|entry| entry.field
                                                           .signature()
                                                           .type_desc()
                                                           .value_type()
                                                           .default_value())
                                         .collect();

        // check for method duplicates
        let mut methods = Vec::new();
        let mut present_methods: HashSet<MethodSig> = HashSet::with_capacity(m.size_hint().0);

        for method in m {
            if present_methods.contains(method.signature()) {
                return Err(ClassError::DuplicateMethod(method.signature().clone()));
            }

            present_methods.insert(method.signature().clone());
            methods.push(Arc::new(method));
        }

        Ok(Class {
            name,
            fields,
            methods,
            static_fields_values: RwLock::new(static_fields_values),
            nonstatic_fields_len,
        })
    }

    /// Returns the name of this class.
    pub fn name(&self) -> &ClassName {
        &self.name
    }
}


/// Fields.
impl Class {
    /// Returns an iterator over all fields.
    pub fn fields(&self) -> impl ExactSizeIterator<Item=&Arc<Field>> {
        self.fields.iter().map(|entry| &entry.field)
    }

    /// Finds a field of the given signature.
    ///
    /// # Errors
    ///
    /// Returns a `ClassError::NoSuchField` if there is not a field of the given signature.
    pub fn field(&self, signature: &FieldSig) -> Result<&Arc<Field>, ClassError> {
        Ok(&self.field_entry(signature)?.field)
    }

    /// Finds a static field of the given signature.
    ///
    /// # Errors
    ///
    /// Returns a `ClassError::NoSuchField` if there is not a static field of the given signature.
    pub fn static_field(&self, signature: &FieldSig) -> Result<&Arc<Field>, ClassError> {
        Ok(&self.static_field_entry(signature)?.field)
    }

    /// Finds an instance field of the given signature.
    ///
    /// # Errors
    ///
    /// Returns a `ClassError::NoSuchField` if there is not an instance field of the given signature.
    pub fn instance_field(&self, signature: &FieldSig) -> Result<&Arc<Field>, ClassError> {
        Ok(&self.static_field_entry(signature)?.field)
    }
}


/// Field entries.
impl Class {
    /// Finds a field entry of the field of given signature.
    ///
    /// # Errors
    ///
    /// Returns a `ClassError::NoSuchField` if there is not a field of the given signature.
    fn field_entry(&self, signature: &FieldSig) -> Result<&FieldEntry, ClassError> {
        self.fields.iter()
            .find(|entry| {
                entry.field.signature() == signature
            })
            .ok_or(ClassError::NoSuchField(signature.clone()))
    }

    /// Finds a static field entry.
    ///
    /// # Errors
    ///
    /// Returns a `ClassError::NoSuchField` if there is not a static field of the given signature.
    fn static_field_entry(&self, signature: &FieldSig) -> Result<&FieldEntry, ClassError> {
        let entry = self.field_entry(&signature)?;

        if !entry.field.is_static() {
            return Err(ClassError::NoSuchField(signature.clone()));
        }

        Ok(entry)
    }

    /// Finds an instance field entry.
    ///
    /// # Errors
    ///
    /// Returns a `ClassError::NoSuchField` if there is not an instance field of the given signature.
    fn instance_field_entry(&self, signature: &FieldSig) -> Result<&FieldEntry, ClassError> {
        let entry = self.field_entry(&signature)?;

        if entry.field.is_static() {
            return Err(ClassError::NoSuchField(signature.clone()));
        }

        Ok(entry)
    }
}


/// Field values.
impl Class {
    /// Returns a static field value.
    ///
    /// # Errors
    ///
    /// Returns a `ClassError::NoSuchField` if there is not a static field of the given signature.
    pub fn static_field_value(&self, signature: &FieldSig) -> Result<Value, ClassError> {
        let i = self.static_field_entry(signature)?.offset;
        Ok(self.static_fields_values.read().unwrap()[i].clone())
    }

    /// Sets a static field value.
    ///
    /// # Errors
    ///
    /// Returns a `ClassError::NoSuchField` if there is not a static field of the given signature.
    pub fn set_static_field_value(&self, signature: &FieldSig, value: Value) -> Result<(), ClassError> {
        let i = self.static_field_entry(signature)?.offset;

        if !signature.type_desc().is_assignable_with(&value) {
            return Err(ClassError::FieldValueTypeMismatch(signature.clone(), value));
        }

        self.static_fields_values.write().unwrap()[i] = value;
        Ok(())
    }

    /// Returns an instance field value.
    ///
    /// # Errors
    ///
    /// Returns a `ClassError::NoSuchField` if there is not a static field of the given signature.
    /// Returns a `ClassError::NotInstanceOf` if the given instance is not an instance of this class.
    pub fn instance_field_value(&self, instance: &Instance, signature: &FieldSig) -> Result<Value, ClassError> {
        if self.name != instance.class().name {
            return Err(ClassError::NotInstanceOf(instance.class().name.clone(), self.name.clone()));
        }

        let i = self.instance_field_entry(signature)?.offset;
        Ok(instance.field(i))
    }

    /// Sets an instance field value.
    ///
    /// # Errors
    ///
    /// Returns a `ClassError::NoSuchField` if there is not a instance field of the given signature.
    /// Returns a `ClassError::NotInstanceOf` if the given instance is not an instance of this class.
    pub fn set_instance_field_value(&self, instance: &Instance, signature: &FieldSig, value: Value) -> Result<(), ClassError> {
        if self.name != instance.class().name {
            return Err(ClassError::NotInstanceOf(instance.class().name.clone(), self.name.clone()));
        }

        let i = self.instance_field_entry(signature)?.offset;

        if !signature.type_desc().is_assignable_with(&value) {
            return Err(ClassError::FieldValueTypeMismatch(signature.clone(), value));
        }

        instance.set_field(i, value);
        Ok(())
    }
}


/// Methods.
impl Class {
    /// Returns an iterator over all methods.
    pub fn methods(&self) -> impl ExactSizeIterator<Item=&Arc<Method>> {
        self.methods.iter()
    }

    /// Finds a method of the given signature.
    ///
    /// # Errors
    ///
    /// Returns a `ClassError::NoSuchMethod` if there is not a field of the given signature.
    fn method(&self, signature: &MethodSig) -> Result<&Arc<Method>, ClassError> {
        self.methods.iter()
            .find(|method| {
                method.signature() == signature
            })
            .ok_or(ClassError::NoSuchMethod(signature.clone()))
    }

    /// Finds a static method of the given signature.
    ///
    /// # Errors
    ///
    /// Returns a `ClassError::NoSuchMethod` if there is not a static method of the given signature.
    pub fn static_method(&self, signature: &MethodSig) -> Result<&Arc<Method>, ClassError> {
        let method = self.method(&signature)?;

        if !method.is_static() {
            return Err(ClassError::NoSuchMethod(signature.clone()));
        }

        Ok(method)
    }

    /// Finds an instance method of the given signature.
    ///
    /// # Errors
    ///
    /// Returns a `ClassError::NoSuchMethod` if there is not an instance method of the given signature.
    pub fn instance_method(&self, signature: &MethodSig) -> Result<&Arc<Method>, ClassError> {
        let method = self.method(&signature)?;

        if method.is_static() {
            return Err(ClassError::NoSuchMethod(signature.clone()));
        }

        Ok(method)
    }
}


#[derive(Debug, Clone)]
struct FieldEntry {
    offset: usize,
    field: Arc<Field>,
}