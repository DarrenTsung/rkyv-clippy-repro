use rkyv::*;

#[derive(Archive, Deserialize, Serialize)]
pub struct Foo(u32);

#[derive(Archive, Deserialize, Serialize)]
pub struct Bar {
    id: u32,
}
