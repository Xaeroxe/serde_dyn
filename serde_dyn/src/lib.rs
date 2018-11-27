extern crate fnv;
extern crate serde;

mod uuid;

use fnv::FnvHashMap as HashMap;
use serde::de::{DeserializeOwned, Deserializer};
use std::any::Any;

/// Provides a statically defined UUID for a Rust type.  It's recommended to implement this
/// by generating a v4 UUID, and transmuting it into a `u128`.  Here's an example of how to do so
///
/// ```
/// extern crate uuid;
/// use std::mem::transmute;
/// use uuid::Uuid;

/// fn main() {
///     println!("{}", unsafe {transmute::<[u8; 16], u128>(*Uuid::new_v4().as_bytes())});
/// }
/// ```
///
/// All types registered with the `TUSM` must have a unique value provided for this trait.
pub trait TypeUuid {
    const UUID: u128;
}

/// Allows the TypeUuid constants to be retrieved via a trait object.  It is automatically implemented
/// for all types that implement TypeUuid.
/// 
/// It is theoretically possible to manually implement this independent of `TypeUuid`.  Please don't.
/// It is critical that this return value be deterministic, and manual implementation could prevent that.
pub trait TypeUuidDynamic {
    fn uuid(&self) -> u128;
}

impl<T: TypeUuid> TypeUuidDynamic for T {
    fn uuid(&self) -> u128 {
        Self::UUID
    }
}

/// TUSM aka Type Uuid Serde Mapper
///
/// This structure maps Type Uuids to Serde functions
#[derive(Clone, Debug)]
pub struct TUSM<'de, D>
where
    D: Deserializer<'de>,
{
    mapping: HashMap<u128, fn(D) -> Result<Box<dyn Any>, D::Error>>,
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
    pub fn manually_register(&mut self, uuid: u128, function: fn(D) -> Result<Box<dyn Any>, D::Error>)
    {
        self.mapping.insert(uuid, function);
    }

    /// Using the provided UUID, attempt to deserialize the next value according to previously
    /// registered mappings.  If no registration for the UUID is found, this will panic.
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
