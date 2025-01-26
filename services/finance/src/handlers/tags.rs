use http_problems::Result;

use crate::domains::tags::Tag;

use super::Handler;

impl Handler {
    pub async fn list_tags(&self) -> Result<Vec<Tag>> {
        self.tags_repository.list_tags().await
    }

    pub async fn insert_many_tags(&self, tags: Vec<String>) -> Result<Vec<Tag>> {
        let exists_tags = self.tags_repository.list_tags().await?;

        let filtered_tags: Vec<String> = tags
            .into_iter()
            .filter(|tag| {
                !exists_tags
                    .iter()
                    .any(|existing_tag| existing_tag.value == *tag)
            })
            .collect();

        if filtered_tags.is_empty() {
            return Ok(exists_tags);
        }

        self.tags_repository.insert_many_tags(filtered_tags).await
    }
}
