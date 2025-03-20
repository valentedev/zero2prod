//! src/routes/subscription.rs
use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// This is a function decoration that handles the Tracing Instrumentation in separation to the function the focuses in the actual business logic
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        //request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // if !is_valid_name(&form.name) {
    //     return HttpResponse::BadRequest().finish();
    // }

    // let name = match SubscriberName::parse(form.0.name) {
    //     Ok(name) => name,
    //     Err(_) => return HttpResponse::BadRequest().finish()
    // };

    // let email = match SubscriberEmail::parse(form.0.email) {
    //     Ok(email) => email,
    //     Err(_) => return HttpResponse::BadRequest().finish()
    // };

    // let new_subscriber = NewSubscriber {
    //     email,
    //     name,
    // };

    let new_subscriber = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    // let new_subscriber = match NewSubscriber::try_from(form.0) {
    //     Ok(form) => form,
    //     Err(_) => return HttpResponse::BadRequest().finish(),
    // };

    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// pub fn parse_subscriber(form: FormData) -> Result<NewSubscriber, String> {
// let name = SubscriberName::parse(form.name)?;
// let email = SubscriberEmail::parse(form.email)?;
// Ok(NewSubscriber {name, email})
// }

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { name, email })
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
// this function takes care of the database logic and has no awareness of the surronding web framework (e.g. web::Form)
//pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
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

// fn is_valid_name(s: &str) -> bool {
//     let is_empty_or_whitespace = s.trim().is_empty();
//     let is_too_long = s.graphemes(true).count() > 256;
//     let forbiden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
//     let contains_forbidden_characters = s.chars().any(|g| forbiden_characters.contains(&g));

//     !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
// }

// pub async fn subscribe(
//     form: web::Form<FormData>,
//     pool: web::Data<PgPool>,
// ) -> HttpResponse {
// let request_id = Uuid::new_v4();
// let request_span = tracing::info_span!(
//     "Adding a new subscriber.",
//     %request_id,
//     subscriber_email = %form.email,
//     subscriber_name = %form.name
// );
// let _request_span_guard = request_span.enter();
// let query_span = tracing::info_span!("Saving new subscriber details in the database");
// tracing::info!(
//     "request_id {} - Adding '{}' '{}' as a new subscriber.",
//     request_id,
//     form.email,
//     form.name
// );
// tracing::info!(
//     "request_id {} - Saving new subscriber details in the database",
//     request_id
// );
// match sqlx::query!(
//     r#"
// INSERT INTO subscriptions (id, email, name, subscribed_at)
// VALUES ($1, $2, $3, $4)
//         "#,
//     Uuid::new_v4(),
//     form.email,
//     form.name,
//     Utc::now()
// )
// .execute(pool.get_ref())
// .instrument(query_span)
// .await
// {
//     Ok(_) => {
//         tracing::info!(
//             "request_id {} - New subscriber details have been saved",
//             request_id
//         );
//         HttpResponse::Ok().finish()
//     }
//     Err(e) => {
//         // println!("Failed to execute query: {}", e);
//         tracing::error!(
//             "request_id {} - Failed to execute query: {:?}",
//             request_id,
//             e
//         );
//         HttpResponse::InternalServerError().finish()
//     }
// }
//}
