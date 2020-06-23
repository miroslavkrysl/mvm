use std::collections::HashMap;
use std::sync::Arc;

use crate::class::{ClassError, ClassFlags, ClassName, ConstantPool, Field, FieldDescriptor, FieldId, FieldName, FieldSymRef, Method, MethodDescriptor, MethodId, MethodName, MethodSymRef, FieldFlags};
use std::collections::hash_map::Entry;

#[derive(Debug, Clone)]
pub struct FieldInfo {
    index: usize,
    field: Field
}

impl FieldInfo {
    pub fn new(index: usize, field: Field) -> Self {
        FieldInfo { index, field }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn field(&self) -> &Field {
        &self.field
    }
}


#[derive(Debug, Clone)]
pub struct Class {
    name: ClassName,
    flags: ClassFlags,
    super_class: Option<Arc<Class>>,
    constant_pool: ConstantPool,
    fields: HashMap<FieldId, FieldInfo>,
    static_fields_count: usize,
    instance_fields_count: usize,
    methods: HashMap<MethodId, Method>,
}

impl Class {
    pub fn new(
        name: ClassName,
        flags: ClassFlags,
        super_class: Option<Arc<Class>>,
        constant_pool: ConstantPool,
        fields: Vec<Field>,
        methods: Vec<Method>,
    ) -> Result<Self, ClassError> {

        // count, enumerate and check uniqueness of the fields

        let mut static_fields_count = 0;
        let mut instance_fields_count = 0;

        let mut fields_map = HashMap::with_capacity(fields.len());

        for field in fields.into_iter() {
            let index;

            if field.flags().has(FieldFlags::STATIC) {
                index = static_fields_count;
                static_fields_count += 1;
            } else {
                index = instance_fields_count;
                instance_fields_count += 1;
            }

            let id = FieldId::new(field.name().clone(), field.descriptor().clone());
            let field_info = FieldInfo::new(index, field);

            match fields_map.entry(id) {
                Entry::Occupied(_) => return Err(ClassError::MultipleFieldDefinitions),
                Entry::Vacant(entry) => entry.insert(field_info),
            };
        };


        // check uniqueness of the methods

        let mut methods_map = HashMap::with_capacity(fields.len());

        for method in methods.into_iter() {
            let id = MethodId::new(method.name().clone(), method.descriptor().clone());

            match methods_map.entry(id) {
                Entry::Occupied(_) => return Err(ClassError::MultipleMethodDefinitions),
                Entry::Vacant(entry) => entry.insert(method),
            };
        }

        Ok(Class {
            name,
            flags,
            super_class,
            constant_pool,
            fields: fields_map,
            static_fields_count,
            instance_fields_count,
            methods: methods_map,
        })
    }

    pub fn flags(&self) -> &ClassFlags {
        &self.flags
    }

    pub fn constant_pool(&self) -> &ConstantPool {
        &self.constant_pool
    }

    pub fn method(&self, id: &MethodId) -> Result<&Method, ClassError> {
        self.methods.get(id).ok_or(ClassError::NoSuchMethodFound)
    }

    pub fn field_info(&self, id: &FieldId) -> Result<&FieldInfo, ClassError> {
        self.fields.get(id)
            .ok_or(ClassError::NoSuchFieldFound)
    }
}
