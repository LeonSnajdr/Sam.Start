use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::db;
use crate::placeholder::placeholder_enums::{PlaceholderKind, PlaceholderSource, PlaceholderVisibility};
use crate::placeholder::placeholder_models::{PlaceholderModel, PlaceholderUpdateModel};
use crate::prelude::*;

pub async fn create(create_model: PlaceholderModel) -> Result<PlaceholderModel> {
	let placeholder = sqlx::query_as!(
		PlaceholderModel,
		r#"--sql
            insert into placeholder
                (id, project_id, name, value, visibility, kind, source, date_created, date_last_updated)
                values
                ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            returning
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                value,
                visibility as "visibility: PlaceholderVisibility",
                kind as "kind: PlaceholderKind",
                source as "source: PlaceholderSource",
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
        "#,
		create_model.id,
		create_model.project_id,
		create_model.name,
		create_model.value,
		create_model.visibility,
		create_model.kind,
		create_model.source,
		create_model.date_created,
		create_model.date_last_updated
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(placeholder)
}

pub async fn get_many(project_id: Uuid) -> Result<Vec<PlaceholderModel>> {
	let placeholders = sqlx::query_as!(
		PlaceholderModel,
		r#"--sql
            select
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                value,
                visibility as "visibility: PlaceholderVisibility",
                kind as "kind: PlaceholderKind",
                source as "source: PlaceholderSource",
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
            from placeholder
            where
            	project_id is $1 or
            	(project_id is not $1 and visibility is $2)
            order by name desc
        "#,
		project_id,
		PlaceholderVisibility::Global
	)
	.fetch_all(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(placeholders)
}

pub async fn get_one(id: Uuid) -> Result<PlaceholderModel> {
	let placeholder = sqlx::query_as!(
		PlaceholderModel,
		r#"--sql
            select
                id as "id: Uuid",
                project_id as "project_id: Uuid",
                name,
                value,
                visibility as "visibility: PlaceholderVisibility",
                kind as "kind: PlaceholderKind",
                source as "source: PlaceholderSource",
                date_created as "date_created: DateTime<Utc>",
                date_last_updated as "date_last_updated: DateTime<Utc>"
            from placeholder
            where id = $1
        "#,
		id
	)
	.fetch_one(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(placeholder)
}

pub async fn update_one(update_container: PlaceholderUpdateModel) -> Result<()> {
	sqlx::query!(
		r#"--sql
            update placeholder
            set
                project_id = $1,
                name = $2,
                value = $3,
                visibility = $4,
                kind = $5,
                source = $6,
                date_last_updated = $7
            where id = $8
        "#,
		update_container.project_id,
		update_container.name,
		update_container.value,
		update_container.visibility,
		update_container.kind,
		update_container.source,
		update_container.date_last_updated,
		update_container.id
	)
	.execute(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(())
}

pub async fn delete_one(id: Uuid) -> Result<()> {
	sqlx::query!(
		r#"--sql
            delete
            from placeholder
            where id = $1
        "#,
		id
	)
	.execute(db::get_pool())
	.await
	.map_err(|err| Error::Db(err.to_string()))?;

	Ok(())
}
