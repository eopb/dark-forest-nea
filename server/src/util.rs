use {bson::document::Document, serde::Serialize};

pub trait BsonDoc: Serialize {
    fn as_bson(&self) -> bson::ser::Result<Document> {
        if let bson::Bson::Document(document) = bson::to_bson(self)? {
            Ok(document)
        } else {
            unreachable!("Type should be convertible to document.");
        }
    }
}
