pub mod daemon;
pub mod general;
pub mod project;

use crate::error::ProjectResult;

type StStrA = &'static [&'static str];

pub trait CliUtil where
    Self: Sized,
{
    const NAMES : StStrA;
    fn run() -> ProjectResult<()>;
    fn add_arg(self) -> Self { self }
}
