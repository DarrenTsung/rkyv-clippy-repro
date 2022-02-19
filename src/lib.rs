use rkyv::*;

macro_rules! generate_enums {
    ($($enum_name:ident$(,)*)*) => {
        $(
            #[derive(Archive)]
            pub enum $enum_name {
                HasInput,
            }
        )*
    };
}

generate_enums!(Foo, Baz);

#[derive(Archive)]
pub enum Bar {
    HasInput,
}
