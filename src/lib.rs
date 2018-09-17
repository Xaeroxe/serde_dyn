extern crate serde;

use serde::de::{Deserializer, DeserializeOwned};
use std::any::Any;
use std::collections::HashMap;

pub trait TypeUuid {
    const UUID: u128;
}

pub trait DeserializeDyn: DeserializeOwned + Any {
    fn deserialize_dyn<'de, D>(deserializer: D) -> Result<Box<dyn Any>, D::Error>
    where
        D: Deserializer<'de>;
}

impl<T> DeserializeDyn for T
where
    T: DeserializeOwned + Any,
{
    fn deserialize_dyn<'de, D>(deserializer: D) -> Result<Box<dyn Any>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::deserialize(deserializer).map(|i| Box::new(i) as Box<dyn Any>)
    }
}

/// TUSM aka Type Uuid Serde Mapper
///
/// This structure maps Type Guids to Serde functions
pub struct TUSM<'de, D> where D: Deserializer<'de> {
    mapping: HashMap<u128, fn(D) -> Result<Box<dyn Any>, D::Error>>,
}

impl<'de, D> TUSM<'de, D> where D: Deserializer<'de> {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    pub fn register<T: DeserializeDyn + TypeUuid>(&mut self) {
        self.mapping.insert(T::UUID, T::deserialize_dyn);
    }

    pub fn deserialize_with_uuid(
        &self,
        uuid: &u128,
        deserializer: D,
    ) -> Result<Box<dyn Any>, D::Error> {
        self.mapping
            .get(&uuid)
            .expect("Type not registered!  Please register this type first.")(deserializer)
    }
}

#[cfg(test)]
mod tests {
    extern crate ron;

    use super::*;

    impl TypeUuid for i32 {
        const UUID: u128 = 1;
    }

    #[test]
    fn deser_test() {
        let mut deserializer = ron::de::Deserializer::from_str("5").unwrap();
        let mut tusm = TUSM::new();

        tusm.register::<i32>();


        let new_value = *tusm.deserialize_with_uuid(
            &i32::UUID,
            &mut deserializer,
        ).unwrap().downcast::<i32>().unwrap();
        assert_eq!(new_value, 5);
    }
}
