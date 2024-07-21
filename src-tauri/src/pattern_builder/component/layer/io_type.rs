use std::any::{Any, TypeId};
use std::cmp::{PartialEq};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::repeat;
use itertools::Itertools;

#[derive(Copy, Clone, Debug, Default)]
struct OptionLevel(u16);
#[derive(Copy, Clone, Debug)]
struct BaseTypeId(TypeId);

pub trait DynType: Sized + Send + Sync + 'static {
    fn dyn_type_info() -> &'static DynTypeInfo;
    fn dyn_type_def() -> DynTypeDef {
        DynTypeDef {
            base_type_id: TypeId::of::<Self>(),
            info: Self::dyn_type_info(),
            option_level: 0,
        }
    }
    fn into_dyn_value(self) -> DynValue {
        DynValue {
            value: Box::new(self),
            type_def: Self::dyn_type_def(),
        }
    }
}

impl<T> DynType for Option<T> where T: DynType
{
    fn dyn_type_info() -> &'static DynTypeInfo {
        T::dyn_type_info()
    }
    fn dyn_type_def() -> DynTypeDef {
        let mut def = T::dyn_type_def();
        def.option_level += 1;
        def
    }
}


#[macro_export]
macro_rules! impl_dyn_type {
    ($ty:ty: $type_def:expr) => {
        impl $crate::pattern_builder::component::layer::io_type::DynType for $ty {
            fn dyn_type_info() -> &'static $crate::pattern_builder::component::layer::io_type::DynTypeInfo {
                &$type_def
            }
        }
    }
}
#[macro_export]
macro_rules! static_dyn_type_def {
    ($i:ident = $type_def:expr) => {
        static $i: once_cell::sync::Lazy<$crate::pattern_builder::component::layer::io_type::DynTypeInfo> = once_cell::sync::Lazy::new(|| {
            $type_def
        });
    }
}

#[derive(Clone)]
pub struct DynTypeInfo {
    name: String,
}

impl DynTypeInfo {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct DynTypeDef {
    base_type_id: TypeId,
    option_level: u16,
    info: &'static DynTypeInfo,
}

impl DynTypeDef {
    pub fn name(&self) -> String {
        format!("{}{}", self.info.name, repeat("?").take(self.option_level as usize).join(""))
    }
    pub fn option(&self) -> DynTypeDef {
        DynTypeDef {
            base_type_id: self.base_type_id,
            option_level: self.option_level + 1,
            info: self.info,
        }
    }
}

impl Hash for DynTypeDef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_type_id.hash(state);
        self.option_level.hash(state);
    }
}

impl PartialEq for DynTypeDef {
    fn eq(&self, other: &Self) -> bool {
        self.base_type_id == other.base_type_id && self.option_level == other.option_level
    }
}

impl Eq for DynTypeDef {}

#[derive(Default)]
pub struct DynTypeMapper {
    mappers: HashMap<(DynTypeDef, DynTypeDef), Box<dyn Fn(Box<dyn Any>) -> Box<dyn Any> + Send + Sync>>,
}

impl DynTypeMapper {
    pub fn add_basic_mappings<T: DynType>(&mut self) {
        self.set_mapping(|val: T| val);
        self.set_mapping(|_val: ()| None::<T>);
    }
    pub fn set_mapping<I: DynType, O: DynType>(&mut self, mapper: fn(I) -> O) {
        self.mappers.insert(
            (I::dyn_type_def(), O::dyn_type_def()),
            Box::new(move |value: Box<dyn Any>| Box::new(mapper(*value.downcast().unwrap())))
        );
        self.mappers.insert(
            (I::dyn_type_def(), O::dyn_type_def().option()),
            Box::new(move |value: Box<dyn Any>| Box::new(Some(mapper(*value.downcast().unwrap()))))
        );
    }

    pub fn assert_has_mapping(&self, from: DynTypeDef, into: DynTypeDef) -> Result<(), NoMappingError> {
        if self.has_mapping(from, into) {
            Ok(())
        } else {
            Err(NoMappingError::new(from, into))
        }
    }

    pub fn has_mapping(&self, from: DynTypeDef, into: DynTypeDef) -> bool {
        self.mappers.contains_key(&(from, into))
    }
}

impl Debug for DynTypeMapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DynTypeMapper [ {} ]", self.mappers.keys().map(|(i, o)| format!("{} -> {}", i.name(), o.name())).join(", "))
    }
}

pub struct DynValue {
    value: Box<dyn Any>,
    type_def: DynTypeDef,
}

impl DynValue {
    pub fn dyn_type_def(&self) -> DynTypeDef {
        self.type_def
    }

    pub fn try_into<T: DynType>(self, type_mapper: &DynTypeMapper) -> Result<T, MappingFailedError> {
        if let Some(mapper) = type_mapper.mappers.get(&(self.dyn_type_def(), T::dyn_type_def())) {
            Ok(*mapper(self.value).downcast().unwrap())
        } else {
            Err(MappingFailedError { type_err: NoMappingError::new(self.dyn_type_def(), T::dyn_type_def()), original_value: self })
        }
    }

    pub fn downcast<T: DynType>(self) -> Result<T, DowncastError> {
        if T::dyn_type_def() == self.type_def {
            Ok(*self.value.downcast().unwrap())
        } else {
            Err(DowncastError { target_type: T::dyn_type_def(), value: self })
        }
    }
}

#[derive(Copy, Clone)]
pub struct NoMappingError {
    from: DynTypeDef,
    into: DynTypeDef,
}

impl NoMappingError {
    pub fn new(from: DynTypeDef, into: DynTypeDef) -> Self {
        Self { from, into, }
    }

    pub fn from(&self) -> DynTypeDef {
        self.from
    }

    pub fn into(&self) -> DynTypeDef {
        self.into
    }
}

impl Debug for NoMappingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot convert from {} into {}. No mapping has been specified.", self.from.name(), self.into.name())
    }
}

pub struct MappingFailedError {
    type_err: NoMappingError,
    original_value: DynValue,
}

impl MappingFailedError {
    pub fn err(&self) -> NoMappingError {
        self.type_err
    }

    pub fn into_inner(self) -> DynValue {
        self.original_value
    }
}

impl Debug for MappingFailedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.type_err.fmt(f)
    }
}

pub struct DowncastError {
    target_type: DynTypeDef,
    value: DynValue,
}

impl DowncastError {
    pub fn into_inner(self) -> DynValue {
        self.value
    }
}

impl Debug for DowncastError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value type {} does not match expected type {}.", self.value.type_def.name(), self.target_type.name())
    }
}