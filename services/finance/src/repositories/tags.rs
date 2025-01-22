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
        let max_result = sqlx::query!("SELECT MAX(tag_id) as max_id FROM tags")
            .fetch_one(&self.pool)
            .await?;

        let mut max = max_result.max_id.unwrap_or(0);

        let mut query = r#"
            INSERT INTO tags (tag_id, value) VALUES 
        "#
        .to_string();

        for (i, tag) in tags.iter().enumerate() {
            max += max;

            let value = format!("({}, '{}')", max, tag.replace("'", "''"));
            query.push_str(&value);

            if i != tags.len() - 1 {
                query.push_str(", ");
            }
        }
        query.push_str(" RETURNING *");

        let tags: Vec<Tag> = sqlx::query_as(&query).fetch_all(&self.pool).await?;

        Ok(tags)
    }
}
