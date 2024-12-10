use http_problems::Result;

use crate::domains::tags::Tag;

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait TagRepository {
    async fn list_tags(&self) -> Result<Vec<Tag>>;
    async fn insert_many_tags(&self, tags: Vec<(usize, String)>) -> Result<()>;
}

#[async_trait::async_trait]
impl TagRepository for SqlxRepository {
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

    async fn insert_many_tags(&self, tags: Vec<(usize, String)>) -> Result<()> {
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

            let value = format!("({}, '{}')", max, tag.1.replace("'", "''"));
            query.push_str(&value);

            if i != tags.len() - 1 {
                query.push_str(", ");
            }
        }

        sqlx::query(&query).execute(&self.pool).await?;

        Ok(())
    }
}
