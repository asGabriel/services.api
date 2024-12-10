use http_problems::Result;

use crate::domains::tags::Tag;

use super::Handler;

impl Handler {
    pub async fn list_tags(&self) -> Result<Vec<Tag>> {
        self.tags_repository.list_tags().await
    }

    pub async fn insert_many_tags(&self, tags: Vec<String>) -> Result<()> {
        let new_tags = tags.into_iter().enumerate().map(|(i, t)| (i, t)).collect();

        self.tags_repository.insert_many_tags(new_tags).await
    }
}
