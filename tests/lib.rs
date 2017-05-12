extern crate toolbox_account;

use std::collections::HashMap;

use toolbox_account::*;
use toolbox_account::token::*;


struct InMemoryAuthentication
{
    pub users: HashMap<String, Identity>
}

impl Authentication for InMemoryAuthentication
{
    type T = UsernamePasswordToken;

    fn authenticate(&self, token: UsernamePasswordToken) -> Option<&Identity>
    {
        self.users.get(token.name())
    }
}

#[test]
fn registration()
{
}

#[test]
fn authentication()
{
    let mut in_memory = InMemoryAuthentication { users: HashMap::new() };
    in_memory.users.insert("test".to_owned(), Identity { auth: true });

    let auth_token = UsernamePasswordToken::new("test", "test");
    let identity = in_memory.authenticate(auth_token).unwrap();

    assert!(identity.auth);
}