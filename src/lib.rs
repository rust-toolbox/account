pub mod token;

use token::*;

pub struct Identity
{
    pub auth: bool
}

pub trait Registration
{
    fn register(identity: &Identity) -> Option<Identity>;
}

pub trait Authentication
{
    type T: Token;

    fn authenticate(&self, token: Self::T) -> Option<&Identity>;
}
