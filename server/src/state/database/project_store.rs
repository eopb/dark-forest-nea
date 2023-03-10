use crate::{
    state::{database::Insert, Database},
    util::BsonDoc,
};

use shared::data::Project;

use {
    bson::doc,
    mongodb,
    serde::{Deserialize, Serialize},
    tracing::instrument,
    uuid::Uuid,
};

/// List of a users projects
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectStore {
    /// Primary key.
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub uuid: Uuid,
    pub project: Project,
}

impl ProjectStore {
    #[instrument(level = "trace")]
    pub fn new(uuid: Uuid, project: Project) -> Self {
        Self { uuid, project }
    }
}

impl BsonDoc for ProjectStore {}

impl Database {
    pub fn project_store(&self) -> mongodb::Collection {
        self.main().collection("project_store")
    }
    #[instrument(level = "trace", skip(self), err)]
    pub async fn save_project(&self, project: ProjectStore) -> tide::Result<Insert> {
        Ok({
            let collection = self.project_store();
            let query = doc! { "_id":  project.uuid.to_string() };
            let _users_project_list = collection.find_one(query.clone(), None).await?;
            collection.insert_one(project.as_bson()?, None).await?;
            Insert::Success
        })
    }
}
