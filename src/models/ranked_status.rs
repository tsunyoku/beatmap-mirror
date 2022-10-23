use num_derive::FromPrimitive;

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize, FromPrimitive)]
#[repr(i8)]
pub enum RankedStatus {
    All = -3,
    Graveyard,
    WorkInProgress,
    Pending,
    Ranked,
    Approved,
    Qualified,
    Loved,
}
