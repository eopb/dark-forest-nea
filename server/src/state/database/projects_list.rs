use crate::{
    state::{database::Insert, Database},
    util::BsonDoc,
};

use {
    bson::doc,
    mongodb,
    serde::{Deserialize, Serialize},
};

/// List of a users projects
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectsList {
    /// Primary key.
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub user_name: String,
    pub projects: Vec<String>,
}

impl ProjectsList {
    /// Create a new project list for a user with an initial project.
    pub fn new(user_name: String, first_project: String) -> Self {
        Self {
            user_name,
            projects: vec![first_project],
        }
    }
}

impl BsonDoc for ProjectsList {}

impl Database {
    /// Collection where to store project lists.
    pub fn project_lists(&self) -> mongodb::Collection {
        self.main().collection("projects_lists")
    }
    /// Add project to a users project list.
    pub async fn add_project(&self, user: String, project_name: String) -> tide::Result<Insert> {
        Ok({
            let collection: mongodb::Collection = self.project_lists();
            let query = doc! { "_id": &user};
            let users_project_list = collection.find_one(query.clone(), None).await?;
            if let Some(bson::Bson::Array(users_project_list)) =
                users_project_list.as_ref().and_then(|x| x.get("projects"))
            {
                if users_project_list.contains(&bson::Bson::String(project_name.clone())) {
                    Insert::AlreadyExists
                } else {
                    collection
                        .update_one(
                            query,
                            mongodb::options::UpdateModifications::Document(
                                doc! { "$push": { "projects": project_name } },
                            ),
                            None,
                        )
                        .await?;
                    Insert::Success
                }
            } else {
                collection
                    .insert_one(ProjectsList::new(user, project_name).as_bson()?, None)
                    .await?;
                Insert::Success
            }
        })
    }
}
