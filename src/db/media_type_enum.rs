use diesel_derive_enum::DbEnum;

// define your enum
#[derive(Debug, DbEnum, PartialEq)]
pub enum MediaTypeEnum {
    Default,
    Audio,
    Video,
    Pdf,
    Epub,
    Image
}