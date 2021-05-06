use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

#[derive(Debug, Serialize)]
struct SubscribeResponse {
    pincode: String,
    age_limit: u32,
}

#[derive(Debug, Deserialize)]
struct SubscribeRequest {
    pincode: String,
    age_limit: u32,
    token: String,
}

#[post("/api/v1/subscribe")]
async fn subscribe(
    req: web::Json<SubscribeRequest>,
    db: web::Data<Pool<Sqlite>>,
) -> impl Responder {
    // Check if the sub already exists
    let res = get_sub(&db, &req).await;
    if res.is_ok() {
        println!("Sub already exists");
        let resp = SubscribeResponse {
            age_limit: req.age_limit,
            pincode: req.pincode.clone(),
        };
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(&resp);
    }

    let mut conn = db.acquire().await.unwrap();
    let r = sqlx::query("INSERT INTO subs (pincode, age_limit, reg_token) VALUES (?, ?, ?)")
        .bind(&req.pincode)
        .bind(req.age_limit)
        .bind(&req.token)
        .execute(&mut conn)
        .await;
    match r {
        Ok(_) => {
            let resp = SubscribeResponse {
                age_limit: req.age_limit,
                pincode: req.pincode.clone(),
            };
            HttpResponse::Ok()
                .content_type("application/json")
                .json(&resp)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn get_sub<'a>(
    db: &web::Data<Pool<Sqlite>>,
    sub: &'a SubscribeRequest
) -> Result<&'a SubscribeRequest, sqlx::Error> {
    let mut conn = db.acquire().await?;
    sqlx::query("SELECT * FROM subs WHERE pincode = ? and age_limit=? and reg_token = ?")
        .bind(&sub.pincode)
        .bind(&sub.age_limit)
        .bind(&sub.token)
        .fetch_one(&mut conn)
        .await?;
    Ok(sub)
}
