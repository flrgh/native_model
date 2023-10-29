use bincode::{config, Decode, Encode};
use native_model::{native_model, Model};
pub struct Bincode;

impl<T: bincode::Encode> native_model::Encode<T> for Bincode {
    type Error = bincode::error::EncodeError;
    fn encode(obj: &T) -> Result<Vec<u8>, bincode::error::EncodeError> {
        bincode::encode_to_vec(obj, config::standard())
    }
}

impl<T: bincode::Decode> native_model::Decode<T> for Bincode {
    type Error = bincode::error::DecodeError;
    fn decode(data: Vec<u8>) -> Result<T, bincode::error::DecodeError> {
        bincode::decode_from_slice(&data, config::standard()).map(|(result, _)| result)
    }
}

#[derive(Debug, Encode, Decode)]
#[native_model(id = 1, version = 1, with = Bincode)]
struct Foo1 {
    x: i32,
}

#[derive(Debug, Encode, Decode)]
#[native_model(id = 1, version = 2, with = Bincode, from = Foo1)]
struct Foo2 {
    x: i32,
}

impl From<Foo1> for Foo2 {
    fn from(foo1: Foo1) -> Self {
        Foo2 { x: foo1.x }
    }
}

impl From<Foo2> for Foo1 {
    fn from(foo2: Foo2) -> Self {
        Foo1 { x: foo2.x }
    }
}

#[test]
fn test_simple() {
    assert_eq!(Foo1::native_model_id(), 1);
    assert_eq!(Foo1::native_model_version(), 1);

    assert_eq!(Foo2::native_model_id(), 1);
    assert_eq!(Foo2::native_model_version(), 2);
}