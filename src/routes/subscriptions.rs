use actix_web::{web,HttpResponse};
use sqlx::types::Uuid;
use sqlx::{Error, PgConnection, PgPool};
use sqlx::postgres::PgQueryResult;

use sqlx::types::chrono::Utc;
use tracing::Instrument;


#[derive(serde::Deserialize)]
pub struct FormData{
    email: String,
    name: String
}

#[tracing::instrument(
name = "Adding a new subscriber",
skip(form, pool),
fields(
subscriber_email = %form.email,
subscriber_name = %form.name
)
)]
pub async fn subscribe(form: web::Form<FormData>
                       ,pool:web::Data<PgPool>) -> HttpResponse
{
    match insert_subscriber(&pool, &form).await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[tracing::instrument(
name = "Saving new subscriber details in the database", skip(form, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    form: &FormData,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
form.email,
        form.name,
        Utc::now()
    )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}

// pub async fn subscribe(_form: web::Form<FromData>,pool:web::Data<PgPool>) -> HttpResponse
// {
//     let request_id= Uuid::new_v4();
//
//     let request_span= tracing::info_span!(
//         "Adding a new subscriber "
//         %request_id,
//         subscriber_id = %form.email,
//         subscriber_name = %form.name
//         );
//
//
//
//     let _request_span_guard = request_span.enter();
//
//     let query_span = tracing::info_span!(
//         "Saving new subscriber in dtatabase"
//     );
//
//     tracing::info!(
//         "Adding '{}' '{}' as new subscriber",
//         form.email,
//         form.name
//     );
//     tracing::info!("Saving new subscriber details in the database");
//
//     tracing::info!("request_id {}- saved new subscriber", request_id);
//
//    match  sqlx::query!(
//         r#"
//         INSERT INTO subscriptions(id,email, name,subscribed_at)
//         VALUES($1,$2,$3,$4)
//         "#,
//         Uuid::new_v4(),
//         form.email,
//         form.name,
//         Utc::now()
//     )
//         .execute(pool.get_ref())
//        .instrument(query_span)
//        .await {
//        Ok(_) => {
//    ///        tracing::info!("request_id {}- saved new subscriber", request_id);
//            HttpResponse::Ok().finish()
//
//        },
//        Err(e)=>{
//
//            traking::error!(
//                " request_id {}- failed to execute query {:?}",e);
//            HttpResponse::InternalServerError().finish()
//        }
//
//    }
// }



