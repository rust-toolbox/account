pub trait Token
{
    type N;
    type C;

    fn name(&self) -> &Self::N;
    fn credentials(&self) -> &Self::C;
}

pub struct UsernamePasswordToken
{
    username: String,
    password: String,
    credentials: String
}

impl UsernamePasswordToken
{
    pub fn new<S>(username: S, password: S) -> UsernamePasswordToken
        where S: Into<String>
    {
        let username = username.into();
        let password = password.into();

        UsernamePasswordToken {
            username: username,
            password: password.clone(),
            credentials: password
        }
    }

    pub fn password(&self) -> &String
    {
        &self.password
    }
}

impl Token for UsernamePasswordToken
{
    type N = String;
    type C = String;

    fn name(&self) -> &String
    {
        &self.username
    }

    fn credentials(&self) -> &String
    {
        &self.credentials
    }
}


