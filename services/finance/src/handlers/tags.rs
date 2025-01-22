use http_problems::Result;

use crate::domains::tags::Tag;

use super::Handler;

impl Handler {
    pub async fn list_tags(&self) -> Result<Vec<Tag>> {
        self.tags_repository.list_tags().await
    }

    pub async fn insert_many_tags(&self, mut tags: Vec<String>) -> Result<Vec<Tag>> {
        let exists_tags = self.tags_repository.list_tags().await?;
        for (index, tag) in exists_tags.iter().enumerate() {
            if tags.contains(&tag.value) {
                tags.remove(index);
            }
        }

        self.tags_repository.insert_many_tags(tags.to_vec()).await
    }
}
