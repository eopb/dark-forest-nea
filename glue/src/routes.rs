use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Route {
    Index,
    Explore,
    SignIn,
    CreateAccount,
    NewProject,
    NotFound,
}

impl Default for Route {
    fn default() -> Self {
        Self::NotFound
    }
}

impl Into<&'static str> for Route {
    fn into(self) -> &'static str {
        match self {
            Self::Index => "/",
            Self::Explore => "/explore",
            Self::SignIn => "/sign-in",
            Self::CreateAccount => "/create-account",
            Self::NewProject => "/new-project",
            Self::NotFound => panic!("Can not go to 404 route"),
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path: &str = (*self).into();
        write!(f, "{}", path)
    }
}
