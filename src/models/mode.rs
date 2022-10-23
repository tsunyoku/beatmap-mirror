use num_derive::FromPrimitive;

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize, FromPrimitive)]
#[repr(i8)]
pub enum Mode {
    All = -1,
    Standard,
    Taiko,
    Catch,
    Mania,
}
