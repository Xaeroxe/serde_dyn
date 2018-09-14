extern crate serde;

use serde::{Deserialize, Deserializer};
use std::any::Any;
use std::collections::HashMap;

pub trait TypeUuid {
    const UUID: &'static str;
}

pub trait DeserializeDyn<'de>: Deserialize<'de> + Any {
    fn deserialize_dyn<D>(deserializer: &'de mut D) -> Result<Box<dyn Any>, <&'de mut D as Deserializer<'de>>::Error>
    where
        D: 'de,
        &'de mut D: Deserializer<'de>;
}

impl<'de, T> DeserializeDyn<'de> for T
where
    T: Deserialize<'de> + Any,
{
    fn deserialize_dyn<D>(deserializer: &'de mut D) -> Result<Box<dyn Any>, <&'de mut D as Deserializer<'de>>::Error>
    where
        D: 'de,
        &'de mut D: Deserializer<'de>,
    {
        Self::deserialize(deserializer).map(|i| Box::new(i) as Box<dyn Any>)
    }
}

/// TUSM aka Type Uuid Serde Mapper
///
/// This structure maps Type Guids to Serde functions
pub struct TUSM<'de, D> where D: 'de, &'de mut D: Deserializer<'de> {
    mapping: HashMap<&'static str, fn(&'de mut D) -> Result<Box<dyn Any>, <&'de mut D as Deserializer<'de>>::Error>>,
}

impl<'de, D> TUSM<'de, D> where &'de mut D: Deserializer<'de> {
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
        deserializer: &'de mut D,
    ) -> Result<Box<dyn Any>, <&'de mut D as Deserializer<'de>>::Error> {
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
        let mut deserializer = ron::de::Deserializer::from_str("5").unwrap();
        let mut tusm = TUSM::new();
        tusm.register::<i32>();


        let thing = tusm.deserialize_with_uuid(
            i32::UUID,
            &mut deserializer,
        );
        let thing = thing.unwrap();
        let new_value = *thing.downcast::<i32>().unwrap();
        assert_eq!(new_value, 5);
    }
}
