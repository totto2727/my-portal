use sea_orm::DbErr;

pub type Result<T> = std::result::Result<T, DbErr>;

