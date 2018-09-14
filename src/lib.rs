extern crate serde;

use serde::{Deserialize, Deserializer};
use std::any::Any;
use std::collections::HashMap;

pub trait TypeUuid {
    const UUID: &'static str;
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

/// TUSM aka Type Uuid Serde Mapper
///
/// This structure maps Type Guids to Serde functions
pub struct TUSM<'de, D: Deserializer<'de>> {
    mapping: HashMap<&'static str, fn(D) -> Result<Box<dyn Any>, D::Error>>,
}

impl<'de, D: Deserializer<'de>> TUSM<'de, D> {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    pub fn register<T: DeserializeDyn<'de> + TypeUuid>(&mut self) {
        self.mapping.insert(T::UUID, T::deserialize_dyn);
    }

    pub fn deserialize_with_uuid(
        &self,
        uuid: &str,
        deserializer: D,
    ) -> Result<Box<dyn Any>, D::Error> {
        self.mapping
            .get(uuid)
            .expect("Type not registered!  Please register this type first.")(deserializer)
    }
}

#[cfg(test)]
mod tests {
    extern crate ron;

    use super::*;

    impl TypeUuid for i32 {
        const UUID: &'static str = "i32";
    }

    #[test]
    fn deser_test() {
        let mut tusm = TUSM::new();
        tusm.register::<i32>();
        let mut deserializer = ron::de::Deserializer::from_str("5").unwrap();
        let new_value = *tusm.deserialize_with_uuid(
            i32::UUID,
            &mut deserializer,
        ).unwrap().downcast::<i32>().unwrap();
    }
}
