extern crate fnv;
extern crate serde;
extern crate type_uuid;

use fnv::FnvHashMap as HashMap;
use serde::de::{DeserializeOwned, Deserializer};
use type_uuid::TypeUuid;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Error as FmtError, Formatter};

/// TUSM aka Type Uuid Serde Mapper
///
/// This structure maps Type Uuids to Serde functions
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TUSM<'de, D>
where
    D: Deserializer<'de>,
{
    mapping: HashMap<type_uuid::Bytes, fn(D) -> Result<Box<dyn Any>, D::Error>>,
}

impl<'de, D> TUSM<'de, D>
where
    D: Deserializer<'de>,
{
    pub fn new() -> Self {
        Self {
            mapping: HashMap::default(),
        }
    }

    /// Adds the provided type to the list of types this `TUSM` can deserialize.
    pub fn register<T: DeserializeOwned + Any + TypeUuid>(&mut self) {
        self.manually_register(T::UUID, |deserializer| {
            T::deserialize(deserializer).map(|i| Box::new(i) as Box<dyn Any>)
        });
    }

    /// Adds a mapping entry between the provided UUID and the provided deserialization function.
    ///
    /// Please only use this if absolutely necessary, `register` is the preferred alternative.
    pub fn manually_register(
        &mut self,
        uuid: type_uuid::Bytes,
        function: fn(D) -> Result<Box<dyn Any>, D::Error>,
    ) {
        self.mapping.insert(uuid, function);
    }

    /// Using the provided UUID, attempt to deserialize the next value according to previously
    /// registered mappings.
    pub fn deserialize_with_uuid(
        &self,
        uuid: &type_uuid::Bytes,
        deserializer: D,
    ) -> Result<Box<dyn Any>, SerdeDynError<'de, D>> {
        match self.mapping.get(uuid) {
            Some(f) => f(deserializer).map_err(SerdeDynError::DeserializerError),
            None => Err(SerdeDynError::UuidNotFound),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum SerdeDynError<'de, D: Deserializer<'de>> {
    /// A Uuid was passed in and we didn't have a mapping for it.
    UuidNotFound,
    /// The deserialization function returned an error.
    DeserializerError(D::Error),
}

impl<'de, D: Deserializer<'de>> Debug for SerdeDynError<'de, D> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            SerdeDynError::UuidNotFound => write!(f, "UuidNotFound"),
            SerdeDynError::DeserializerError(ref e) => write!(f, "DeserializerError({:?})", e),
        }
    }
}

impl<'de, D: Deserializer<'de>> Display for SerdeDynError<'de, D> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            SerdeDynError::UuidNotFound => write!(f, "Uuid requested not found in TUSM."),
            SerdeDynError::DeserializerError(ref e) => write!(f, "Deserialization error: {}", e),
        }
    }
}

impl<'de, D: Deserializer<'de>> StdError for SerdeDynError<'de, D> {}

#[cfg(test)]
mod tests {
    extern crate ron;

    use super::*;

    #[test]
    fn deser_test() {
        let mut deserializer = ron::de::Deserializer::from_str("5").unwrap();
        let mut tusm = TUSM::new();

        tusm.register::<i32>();

        let new_value = *tusm
            .deserialize_with_uuid(&i32::UUID, &mut deserializer)
            .unwrap()
            .downcast::<i32>()
            .unwrap();
        assert_eq!(new_value, 5);
    }
}
