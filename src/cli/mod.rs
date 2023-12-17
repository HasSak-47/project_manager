use crate::error::ProjectResult;

type StStrA = &'static [&'static str];

pub trait CliUtil where
    Self: Sized,
{
    fn run(self) -> ProjectResult<()>;
    fn add_arg(self, arg: String) -> Self { self }
}


