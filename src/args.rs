use clap:: {
    Parser,
    ArgEnum
};

use std::fmt;

#[derive(Debug,Parser)]
#[clap(author, version, about)]
pub struct CurrencyArgs{
    #[clap(arg_enum)]
    pub to: EntityType,
    #[clap(arg_enum)]
    pub from: EntityType,
    pub amount: i32
}

#[derive(Debug, ArgEnum, Clone)]
pub enum EntityType {
    /// Pounds 
    GBP,
    /// Yen
    JPY
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EntityType::GBP => write!(f, "GBP"),
            EntityType::JPY => write!(f, "JPY"),
        }
    }
}
