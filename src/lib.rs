use rkyv::*;
use uuid::*;

pub fn foo() {
    let mut serializer = rkyv::ser::serializers::WriteSerializer::new(vec![]);
    Uuid::new_v4()
        .serialize(&mut serializer)
        .expect("can serialize");
    vec![Uuid::new_v4()]
        .serialize(&mut serializer)
        .expect("can serialize");
}
