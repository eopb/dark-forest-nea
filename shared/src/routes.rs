use std::{fmt, string::ToString};

/// An enum for all routes used by both server and client.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Route {
    Index,
    Explore,
    SignIn,
    CreateAccount,
    Users {
        user_name: String,
        nest: Option<UserRoute>,
    },
    NewProject,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum UserRoute {
    Projects(Option<Project>),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Project {
    pub project_name: String,
    pub nest: Option<ProjectRoute>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ProjectRoute {
    Edit,
    Play,
}

impl Default for Route {
    fn default() -> Self {
        Self::Index
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Index => "/".to_owned(),
                Self::Explore => "/explore".to_owned(),
                Self::SignIn => "/sign-in".to_owned(),
                Self::CreateAccount => "/create-account".to_owned(),
                Self::NewProject => "/new-project".to_owned(),
                Self::Users { user_name, nest } =>
                    format!("/users/{}{}", user_name, maybe_show(nest)),
            }
        )
    }
}

impl fmt::Display for UserRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Projects(project) => format!("/projects{}", maybe_show(project)),
            }
        )
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("/{}{}", self.project_name, maybe_show(&self.nest)),
        )
    }
}

impl fmt::Display for ProjectRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Edit => "/edit",
                Self::Play => "/play",
            }
        )
    }
}

pub trait SubRoute: fmt::Display {
    /// Title to display on browser tab.
    fn title(&self) -> String;
    fn nest<T: SubRoute>(nest: &Option<T>) -> String {
        maybe_show(&nest.as_ref().map(SubRoute::title))
    }
}

fn maybe_show<T: fmt::Display>(s: &Option<T>) -> String {
    s.as_ref().map_or(String::new(), ToString::to_string)
}

impl SubRoute for Route {
    fn title(&self) -> String {
        match self {
            Self::Index => "Dark Forest".to_owned(),
            Self::Explore => "Explore Dark Forest".to_owned(),
            Self::SignIn => "Sign In".to_owned(),
            Self::CreateAccount => "Create Account".to_owned(),
            Self::NewProject => "New Project".to_owned(),
            Self::Users { user_name, nest } => {
                format!("{}{}", user_name.to_owned(), Self::nest(nest),)
            }
        }
    }
}

impl SubRoute for UserRoute {
    fn title(&self) -> String {
        match self {
            Self::Projects(project) => format!(": Projects: {}", Self::nest(project),),
        }
    }
}

impl SubRoute for Project {
    fn title(&self) -> String {
        let Self { project_name, nest } = self;
        format!("{}{}", project_name, Self::nest(nest),)
    }
}

impl SubRoute for ProjectRoute {
    fn title(&self) -> String {
        match self {
            Self::Edit => ": Edit".to_owned(),
            Self::Play => ": Play".to_owned(),
        }
    }
}

//TODO tests
