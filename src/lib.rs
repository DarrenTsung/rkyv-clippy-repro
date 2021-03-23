use rkyv::*;

#[derive(Archive, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Foo(u32);

#[derive(Archive, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Bar {
    id: u32,
}
