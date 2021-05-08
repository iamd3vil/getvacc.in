use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

#[derive(Debug, Serialize)]
struct SubscribeResponse {
    pincode: String,
    age_limit: u32,
}

#[derive(Debug, Deserialize, Clone)]
struct SubscribeRequest {
    pincode: String,
    age: u32,
    token: String,
}

#[post("/api/v1/subscribe")]
async fn subscribe(
    req: web::Json<SubscribeRequest>,
    db: web::Data<Pool<Sqlite>>,
) -> impl Responder {
    if req.age < 18 {
        return HttpResponse::BadRequest()
            .body("Currently COWIN is not open for less than 18 years of age.");
    }
    let age_limit: u32 = get_age_limit(req.age);
    // Check if the sub already exists
    let res = check_sub(&db, &req).await;
    if res.is_ok() {
        let resp = SubscribeResponse {
            age_limit,
            pincode: req.pincode.clone(),
        };
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(&resp);
    }

    let mut conn = db.acquire().await.unwrap();
    let r = sqlx::query("INSERT INTO subs (pincode, age_limit, reg_token) VALUES (?, ?, ?)")
        .bind(&req.pincode)
        .bind(age_limit)
        .bind(&req.token)
        .execute(&mut conn)
        .await;
    match r {
        Ok(_) => {
            let resp = SubscribeResponse {
                age_limit,
                pincode: req.pincode.clone(),
            };
            HttpResponse::Ok()
                .content_type("application/json")
                .json(&resp)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn check_sub(
    db: &web::Data<Pool<Sqlite>>,
    sub: &SubscribeRequest,
) -> Result<(), sqlx::Error> {
    let mut conn = db.acquire().await?;
    sqlx::query("SELECT * FROM subs WHERE pincode = ? and age_limit=? and reg_token = ?")
        .bind(&sub.pincode)
        .bind(get_age_limit(sub.age))
        .bind(&sub.token)
        .fetch_one(&mut conn)
        .await?;
    Ok(())
}

fn get_age_limit(age: u32) -> u32 {
    match age {
        age if age >= 45 => 45,
        age if (18..45).contains(&age) => 18,
        _ => 0,
    }
}
