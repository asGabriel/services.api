use http_problems::Result;
use uuid::Uuid;

use crate::domains::tags::Tag;

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait TagRepository {
    async fn list_tags(&self) -> Result<Vec<Tag>>;
    async fn insert_many_tags(&self, tags: Vec<String>) -> Result<Vec<Tag>>;
    async fn insert_many_entries_tags_relation(&self, tags: Vec<Tag>, entry_id: Uuid)
        -> Result<()>;
}

#[async_trait::async_trait]
impl TagRepository for SqlxRepository {
    async fn insert_many_entries_tags_relation(
        &self,
        tags: Vec<Tag>,
        entry_id: Uuid,
    ) -> Result<()> {
        if tags.is_empty() {
            return Ok(())
        }

        let mut query = r#"
            INSERT INTO entries_tags (entry_id, tag_id) VALUES 
        "#
        .to_string();
        let tags_len = tags.len() - 1;

        for (i, tag) in tags.into_iter().enumerate() {
            let value = format!("('{}', {})", entry_id, tag.tag_id);
            query.push_str(&value);

            if i != tags_len {
                query.push_str(", ");
            }
        }

        sqlx::query(&query).execute(&self.pool).await?;

        Ok(())
    }

    async fn list_tags(&self) -> Result<Vec<Tag>> {
        let tags = sqlx::query_as!(
            Tag,
            r#"
                SELECT * FROM TAGS
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tags)
    }

    async fn insert_many_tags(&self, tags: Vec<String>) -> Result<Vec<Tag>> {
        let mut query = String::from("INSERT INTO tags (value) VALUES ");
        for (i, _) in tags.iter().enumerate() {
            query.push_str(&format!("(${})", i + 1));
            if i != tags.len() - 1 {
                query.push_str(", ");
            }
        }
        query.push_str(" RETURNING *");

        let mut query_builder = sqlx::query_as::<_, Tag>(&query);
        for tag in tags {
            query_builder = query_builder.bind(tag);
        }

        let tags: Vec<Tag> = query_builder.fetch_all(&self.pool).await?;

        Ok(tags)
    }
}
