use std::{iter, slice};
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use itertools::Itertools;
use crate::class::name::{ClassName, FieldName, MethodName};
use crate::class::flags::ClassFlags;
use crate::class::field::Field;
use crate::class::method::Method;
use crate::types::jvm_value::JvmValue;
use crate::class::error::ClassError;
use crate::class::descriptor::{TypeDescriptor, ReturnDescriptor, ParamsDescriptor};


//
// #[derive(Debug, Clone)]
// pub struct FieldInfo {
//     index: usize,
//     field: Field,
// }
//
// impl FieldInfo {
//     pub fn new(index: usize, field: Field) -> Self {
//         FieldInfo { index, field }
//     }
//
//     pub fn index(&self) -> usize {
//         self.index
//     }
//
//     pub fn field(&self) -> &Field {
//         &self.field
//     }
// }


#[derive(Debug)]
pub struct Class {
    name: ClassName,
    fields: Vec<Arc<Field>>,
    methods: Vec<Arc<Method>>,
    fields_offsets: Vec<usize>,
    static_fields_values: RwLock<Vec<JvmValue>>,
    nonstatic_fields_len: usize,
}

impl Class {
    pub fn new(
        name: ClassName,
        mut fields: Vec<Arc<Field>>,
        mut methods: Vec<Arc<Method>>,
    ) -> Result<Self, ClassError> {

        // count indexes of static and nonstatic fields and check for duplicates
        let mut static_fields_len = 0;
        let mut nonstatic_fields_len = 0;

        let mut present_fields: HashSet<(&FieldName, &TypeDescriptor)> = HashSet::with_capacity(fields.len());
        let mut fields_offsets = Vec::with_capacity(fields.capacity());

        for field in fields.iter() {
            if present_fields.contains(&(field.name(), field.descriptor())) {
                return Err(ClassError::DuplicateField);
            }

            present_fields.insert((field.name(), field.descriptor()));

            let offset;

            if field.is_static() {
                offset = static_fields_len;
                static_fields_len += 1;
            } else {
                offset = nonstatic_fields_len;
                nonstatic_fields_len += 1;
            }

            fields_offsets.push(offset);
        }

        // initialize static fields values
        let static_fields_values = fields.iter()
            .filter(|field| field.is_static())
            .map(|field| field.descriptor().default_value())
            .collect();

        // check for method duplicates
        let mut present_methods: HashSet<(&MethodName, &ReturnDescriptor, &ParamsDescriptor)> = HashSet::with_capacity(methods.len());

        for method in methods.iter() {
            if present_methods.contains(&(method.name(), method.return_desc(), method.params_desc())) {
                return Err(ClassError::DuplicateMethod{
                    return_desc: method.return_desc().clone(),
                    name: method.name().clone(),
                    params_desc: method.params_desc().clone()
                });
            }

            present_methods.insert((method.name(), method.return_desc(), method.params_desc()));
        }

        Ok(Class {
            name,
            fields,
            methods,
            fields_offsets,
            static_fields_values: RwLock::new(static_fields_values),
            nonstatic_fields_len,
        })
    }

    pub fn name(&self) -> &ClassName {
        &self.name
    }
}

impl Class {
    // /// Iterator over all fields including the ones in superclasses.
    // pub fn fields(&self) -> FieldsIter {
    //     FieldsIter { class: self, local_fields: self.fields.iter() }
    // }

    /// Total count of instance fields including the ones from superclasses.
    fn instance_fields_len(&self) -> usize {
        self.nonstatic_fields_len
    }
}


/// Method resolution.
impl Class {
    // /// Find a local method.
    // ///
    // /// # Errors
    // ///
    // /// Will return a ClassError::NoSuchMethodFound if there is not a method
    // /// of the given name and descriptor.
    // pub fn local_method(&self, name: &MethodName, descriptor: &MethodDescriptor) -> Result<&Arc<Method>, ClassError> {
    //     self.methods.iter().find(|method| {
    //         method.name() == name && method.descriptor() == descriptor
    //     }).ok_or(ClassError::NoSuchMethod)
    // }
    //
    // /// Find a method in this class or its superclasses.
    // ///
    // /// # Errors
    // ///
    // /// Will return a ClassError::NoSuchMethod if there is not a method
    // /// of the given name and descriptor.
    // pub fn method(&self, name: &MethodName, descriptor: &MethodDescriptor) -> Result<&Arc<Method>, ClassError> {
    //     self.local_method(name, descriptor)
    //         .or_else(|error| {
    //             match &self.super_class {
    //                 None => Err(error),
    //                 Some(sup) => sup.method(name, descriptor),
    //             }
    //         })
    // }
}

/// Field resolution.
impl Class {
    /// Find a local field and its index in the class.
    ///
    /// # Errors
    ///
    /// Will return a ClassError::NoSuchField if there is not a field
    /// of the given name and descriptor.
    pub fn local_field(&self, name: &FieldName, descriptor: &TypeDescriptor) -> Result<(&Arc<Field>, usize), ClassError> {
        self.fields.iter()
            .position(|field| {
                field.name() == name && field.descriptor() == descriptor
            })
            .map(|index| (&self.fields[index], index))
            .ok_or(ClassError::NoSuchField)
    }
    //
    // /// Find a field in this class or its superclasses.
    // ///
    // /// # Errors
    // ///
    // /// Will return a ClassError::NoSuchField if there is not a static field
    // /// of the given name and descriptor.
    // pub fn field(&self, name: &FieldName, descriptor: &TypeDescriptor) -> Result<(&Arc<Field>, usize), ClassError> {
    //     self.local_field(name, descriptor)
    //         .or_else(|error| {
    //             match &self.super_class {
    //                 None => Err(error),
    //                 Some(sup) => sup.field(name, descriptor),
    //             }
    //         })
    // }

    /// Get the field offset in instance fields or static fields, it depends
    /// on what type of field the field_id is referencing.
    ///
    /// # Panics
    ///
    /// Will panic if the field_id is out of bounds.
    pub fn field_offset(&self, field_id: usize) -> usize {
        self.fields_offsets[field_id]
    }

    /// Get the static field value.
    ///
    /// # Panics
    ///
    /// Will panic if the offset is out of bounds.
    pub fn static_field_value(&self, offset: usize) -> JvmValue {
        self.static_fields_values.read().unwrap()[offset].clone()
    }

    /// Set the static field value.
    ///
    /// # Panics
    ///
    /// Will panic if the offset is out of bounds.
    pub fn set_static_field_value(&mut self, index: usize, value: JvmValue) {
        self.static_fields_values.write().unwrap()[index] = value;
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Class {}

//
// #[derive(Debug, Clone)]
// pub struct FieldsIter<'a> {
//     class: &'a Class,
//     local_fields: slice::Iter<'a, Arc<Field>>,
// }
//
// impl<'a> Iterator for FieldsIter<'a> {
//     type Item = &'a Arc<Field>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.local_fields.next()
//             .or_else(|| {
//                 self.class.super_class().and_then(|sup| {
//                     self.class = sup;
//                     self.local_fields = self.class.fields.iter();
//                     self.next()
//                 })
//             })
//     }
// }
//
// impl<'a> ExactSizeIterator for FieldsIter<'a> {
//     fn len(&self) -> usize {
//         self.class.instance_fields_len()
//     }
// }