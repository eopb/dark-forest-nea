use crate::{
    state::{database::Insert, Database},
    util::BsonDoc,
};

use {
    bson::doc,
    mongodb,
    once_cell::sync::Lazy,
    serde::{Deserialize, Serialize},
    tracing::instrument,
    uuid::Uuid,
};

/// List of a users projects
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectsList {
    /// Primary key.
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub user_name: String,
    pub projects: Vec<Project>,
}

impl ProjectsList {
    /// Create a new project list for a user with an initial project.
    pub fn new(user_name: String, first_project: Project) -> Self {
        Self {
            user_name,
            projects: vec![first_project],
        }
    }
}

impl BsonDoc for ProjectsList {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    name: String,
    uuid: Uuid,
}

impl BsonDoc for Project {}
impl Database {
    /// Collection where to store project lists.
    pub fn project_lists(&self) -> mongodb::Collection {
        self.main().collection("projects_lists")
    }
    /// Add project to a users project list.
    #[instrument(level = "trace", skip(self), err)]
    pub async fn add_project(&self, user: String, project_name: String) -> tide::Result<Insert> {
        let uuid = Lazy::new(Uuid::new_v4);
        Ok({
            let collection = self.project_lists();
            let query = doc! { "_id": &user};
            let users_project_list = collection.find_one(query.clone(), None).await?;
            if let Some(bson::Bson::Array(users_project_list)) =
                users_project_list.as_ref().and_then(|x| x.get("projects"))
            {
                if users_project_list
                    .iter()
                    .map(|bson| bson::from_bson::<Project>(bson.clone()).unwrap())
                    .any(|project| project_name == project.name)
                {
                    Insert::AlreadyExists
                } else {
                    collection
                        .update_one(
                            query,
                            mongodb::options::UpdateModifications::Document(
                                doc! { "$push": { "projects": Project {
                                    name: project_name,
                                    uuid: *uuid,
                                }.as_bson()? } },
                            ),
                            None,
                        )
                        .await?;
                    Insert::Success
                }
            } else {
                collection
                    .insert_one(
                        ProjectsList::new(user, Project {
                            name: project_name,
                            uuid: *uuid,
                        })
                        .as_bson()?,
                        None,
                    )
                    .await?;
                Insert::Success
            }
        })
    }

    #[allow(clippy::find_map)] // find_map is rubbish in this case
    #[instrument(level = "trace", skip(self), err)]
    pub async fn get_project_uuid(
        &self,
        user: &str,
        project_name: &str,
    ) -> tide::Result<Option<Uuid>> {
        let collection = self.project_lists();
        let query = doc! { "_id": &user};
        let users_project_list = collection.find_one(query.clone(), None).await?;
        Ok(
            if let Some(bson::Bson::Array(users_project_list)) =
                users_project_list.as_ref().and_then(|x| x.get("projects"))
            {
                users_project_list
                    .iter()
                    .map(|bson| (bson::from_bson::<Project>(bson.clone()).unwrap()))
                    .find(|project| (project_name == project.name))
                    .map(|project| project.uuid)
            } else {
                todo!()
            },
        )
    }
}
