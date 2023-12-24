use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};


pub struct IOType<T> where T: 'static {
    type_name: String,
    mappers_from: HashMap<TypeId, Box<dyn Fn(Box<dyn Any>) -> Box<dyn Any> + Send + Sync>>,
    mappers_into: HashMap<TypeId, Box<dyn Fn(Box<dyn Any>) -> Box<dyn Any> + Send + Sync>>,
    marker: std::marker::PhantomData<fn(T) -> T>,
}

impl<T> Display for IOType<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.type_name, f)
    }
}

impl<T> IOType<T> where T: 'static {
    pub fn new(type_name: &str) -> Self {
        let mut new = Self {
            type_name: type_name.to_string(),
            mappers_from: HashMap::new(),
            mappers_into: HashMap::new(),
            marker: std::marker::PhantomData::default(),
        };
        new.add_mapping_from(|x| x);
        new.add_mapping_into(|x| x);
        new
    }

    pub fn add_mapping_from<U: 'static>(&mut self, mapper: fn(U) -> T) {
        self.mappers_from.insert(TypeId::of::<U>(), Box::new(move |value: Box<dyn Any>| Box::new(mapper(*value.downcast().unwrap()))));
    }

    pub fn get_mapping_from<U: 'static>(&self) -> Option<Box<dyn Fn(U) -> T + '_>> {
        self.mappers_from.get(&TypeId::of::<U>()).map(|func| {
            Box::new(|val: U| *func(Box::new(val) as Box<dyn Any>).downcast().unwrap()) as Box<dyn Fn(U) -> T>
        })
    }

    pub fn add_mapping_into<U: 'static>(&mut self, mapper: fn(T) -> U) {
        self.mappers_into.insert(TypeId::of::<U>(), Box::new(move |value: Box<dyn Any>| Box::new(mapper(*value.downcast().unwrap()))));
    }

    pub fn get_mapping_into<U: 'static>(&self) -> Option<Box<dyn Fn(T) -> U + '_>> {
        self.mappers_into.get(&TypeId::of::<U>()).map(|func| {
            Box::new(|val: T| *func(Box::new(val) as Box<dyn Any>).downcast().unwrap()) as Box<dyn Fn(T) -> U>
        })
    }
}

trait PrivateErasedIOType {
    fn io_type_id(&self) -> TypeId;
    fn has_mapping_into(&self, into_type: &dyn ErasedIOType) -> bool;
    fn has_mapping_from(&self, from_type: &dyn ErasedIOType) -> bool;
}

pub trait ErasedIOType : PrivateErasedIOType {
    fn type_name(&self) -> &String;
    fn can_map_into(&self, into_type: &dyn ErasedIOType) -> Result<(), NoMappingError>;
    fn can_map_from(&self, into_type: &dyn ErasedIOType) -> Result<(), NoMappingError>;
}

impl<T: 'static> PrivateErasedIOType for IOType<T> {
    fn io_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn has_mapping_into(&self, into_type: &dyn ErasedIOType) -> bool {
        self.mappers_into.contains_key(&into_type.io_type_id())
    }

    fn has_mapping_from(&self, from_type: &dyn ErasedIOType) -> bool {
        self.mappers_from.contains_key(&from_type.io_type_id())
    }
}

impl<T: 'static> ErasedIOType for IOType<T> {
    fn type_name(&self) -> &String {
        &self.type_name
    }

    fn can_map_into(&self, into_type: &dyn ErasedIOType) -> Result<(), NoMappingError> {
        if self.has_mapping_into(into_type) || into_type.has_mapping_from(self) {
            Ok(())
        } else {
            Err(NoMappingError::new(self.type_name(), into_type.type_name()))
        }
    }

    fn can_map_from(&self, from_type: &dyn ErasedIOType) -> Result<(), NoMappingError> {
        if self.has_mapping_from(from_type) || from_type.has_mapping_into(self) {
            Ok(())
        } else {
            Err(NoMappingError::new(from_type.type_name(), self.type_name()))
        }
    }
}

pub struct ErasedIOValue<'a> {
    type_id: TypeId,
    value: Box<dyn Any>,
    type_name: &'a String,
    mappers_into: &'a HashMap<TypeId, Box<dyn Fn(Box<dyn Any>) -> Box<dyn Any> + Send + Sync>>,
}

impl<'a> ErasedIOValue<'a> {
    pub fn new<T: 'static>(value: T, io_type: &'a IOType<T>) -> Self {
        Self {
            type_id: value.type_id(),
            value: Box::new(value),
            type_name: &io_type.type_name,
            mappers_into: &io_type.mappers_into
        }
    }
    pub fn try_into<U: 'static>(self, target: &IOType<U>) -> Result<U, NoMappingError> {
        self.mappers_into.get(&TypeId::of::<U>())
            .or_else(|| target.mappers_from.get(&self.type_id))
            .map(|mapper| *mapper(self.value).downcast().unwrap())
            .ok_or( NoMappingError::new(&self.type_name, &target.type_name))
    }
}

pub struct NoMappingError {
    from_type_name: String,
    into_type_name: String,
}

impl NoMappingError {
    pub fn new(from_type_name: &str, into_type_name: &str) -> Self {
        Self {
            from_type_name: from_type_name.to_string(),
            into_type_name: into_type_name.to_string(),
        }
    }

    pub fn get_from_type_name(&self) -> &String {
        &self.from_type_name
    }

    pub fn get_into_type_name(&self) -> &String {
        &self.into_type_name
    }
}

impl Debug for NoMappingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot convert from {} into {}. No mapping has been specified.", self.from_type_name, self.into_type_name)
    }
}
