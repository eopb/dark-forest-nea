//! Useful functions not complex enough for their own modules.

use {bson::document::Document, serde::Serialize};

/// Implement this only for types that can be converted into `bson` documents.
pub trait BsonDoc: Serialize {
    fn as_bson(&self) -> bson::ser::Result<Document> {
        if let bson::Bson::Document(document) = bson::to_bson(self)? {
            Ok(document)
        } else {
            unreachable!("Type should be convertible to document.");
        }
    }
}
