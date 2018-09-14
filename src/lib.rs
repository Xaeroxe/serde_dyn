extern crate serde;

use serde::{Deserialize, Deserializer};
use std::any::Any;
use std::collections::HashMap;

pub trait TypeGuid {
    const GUID: &'static str;
}

pub trait DeserializeDyn<'de>: Deserialize<'de> + Any {
    fn deserialize_dyn<D>(deserializer: D) -> Result<Box<dyn Any>, D::Error>
    where
        D: Deserializer<'de>;
}

impl<'de, T> DeserializeDyn<'de> for T
where
    T: Deserialize<'de> + Any,
{
    fn deserialize_dyn<D>(deserializer: D) -> Result<Box<dyn Any>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::deserialize(deserializer).map(|i| Box::new(i) as Box<dyn Any>)
    }
}

/// TGSM aka Type Guid Serde Mapper
///
/// This structure maps Type Guids to Serde functions
pub struct TGSM<'de, D: Deserializer<'de>> {
    mapping: HashMap<&'static str, fn(D) -> Result<Box<dyn Any>, D::Error>>,
}

impl<'de, D: Deserializer<'de>> TGSM<'de, D> {
    pub fn register<T: DeserializeDyn<'de> + TypeGuid>(&mut self) {
        self.mapping.insert(T::GUID, T::deserialize_dyn);
    }

    pub fn deserialize_with_guid(
        &self,
        guid: &str,
        deserializer: D,
    ) -> Result<Box<dyn Any>, D::Error> {
        self.mapping
            .get(guid)
            .expect("Type not registered!  Please register this type first.")(deserializer)
    }
}
