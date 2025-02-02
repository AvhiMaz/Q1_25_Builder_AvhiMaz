use anchor_lang::error_code;

#[error_code]
pub enum MarketPlaceError {
    #[msg("The name is too long")]
    NameTooLong,
    #[msg("The collection is invalid")]
    InvalidCollections,
}
