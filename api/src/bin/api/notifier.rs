use chrono::{Local, Utc};
use getvacc::get_centers;
use sqlx::{sqlite::SqliteRow, Pool, Row, Sqlite};
use std::error::Error;
use std::sync::Arc;
use tokio::time;

const NOTIF_TITLE: &str = "Alert from getvacc.in";

pub struct Notifier {
    fcm_client: Arc<fcm::Client>,
    db: Arc<Pool<Sqlite>>,
    api_key: String,
    period: u32,
}

impl Notifier {
    pub fn new(db: Pool<Sqlite>, fcm_client: fcm::Client, api_key: String, period: u32) -> Self {
        Self {
            db: Arc::new(db),
            fcm_client: Arc::new(fcm_client),
            api_key,
            period,
        }
    }

    pub async fn start_loop(&self) -> Result<(), Box<dyn Error>> {
        let mut interval = time::interval(time::Duration::from_secs(self.period as u64));
        loop {
            println!("Starting notifier loop");
            interval.tick().await;
            // Get all pincodes
            let subs = self.get_all_subs().await?;

            // Loop over each sub and check if there are any slots available.
            subs.iter().for_each(|sub| {
                let sub = sub.clone();
                let db = self.db.clone();
                let fcm_client = self.fcm_client.clone();
                let api_key = self.api_key.clone();
                tokio::spawn(async move {
                    check_slots(&db, &fcm_client, api_key, &sub).await;
                });
            });
        }
    }

    async fn get_all_subs(&self) -> Result<Vec<Sub>, sqlx::Error> {
        let mut subs: Vec<Sub> = vec![];
        let mut conn = self.db.acquire().await?;
        sqlx::query("SELECT DISTINCT pincode, age_limit FROM subs")
            .map(|row: SqliteRow| {
                let sub = Sub {
                    age_limit: row.get("age_limit"),
                    pincode: row.get("pincode"),
                    token: String::from(""), // Don't really need token here.
                };
                subs.push(sub);
            })
            .fetch_all(&mut conn)
            .await?;
        Ok(subs)
    }
}

#[derive(Debug, Clone)]
struct Sub {
    pincode: String,
    age_limit: u32,
    token: String,
}

async fn check_slots(db: &Pool<Sqlite>, fcm_client: &fcm::Client, api_key: String, sub: &Sub) {
    println!("Checking slot for pincode: {}", &sub.pincode);
    let centres = get_centers(&sub.pincode, &get_date()).await.unwrap();
    let available_centres: Vec<&getvacc::Center> = centres
        .centers
        .iter()
        .filter(|c| {
            let mut av = false;
            for ses in &c.sessions {
                if ses.min_age_limit <= sub.age_limit && ses.available_capacity > 0 {
                    av = true
                }
            }
            av
        })
        .collect();
    if !available_centres.is_empty() {
        let res = send_notification(
            db,
            fcm_client,
            api_key,
            &sub,
            make_notification(&available_centres, &sub),
        )
        .await;
        if let Err(e) = res {
            println!(
                "error while sending notification for sub: {}, {:?}",
                e.to_string(),
                sub
            );
        }
    }
}

async fn send_notification(
    db: &Pool<Sqlite>,
    fcm_client: &fcm::Client,
    api_key: String,
    sub: &Sub,
    notif: String,
) -> Result<(), Box<dyn Error>> {
    // Get all tokens for this pincode and send notification.
    let mut tokens: Vec<String> = vec![];
    let mut conn = db.acquire().await?;
    sqlx::query(
        "SELECT reg_token, last_notification_at 
        FROM subs WHERE pincode=? and age_limit=? and ? - last_notification_at >= ?",
    )
    .bind(&sub.pincode)
    .bind(&sub.age_limit)
    .bind(&Utc::now().timestamp())
    .bind(7200) // 2 hours
    .map(|row: SqliteRow| {
        tokens.push(row.get("reg_token"));
    })
    .fetch_all(&mut conn)
    .await?;
    let mut fcm_notif = fcm::NotificationBuilder::new();
    fcm_notif.title(NOTIF_TITLE);
    fcm_notif.body(&notif);

    if tokens.is_empty() {
        return Ok(());
    }

    let mut builder = fcm::MessageBuilder::new_multi(&api_key, &tokens);
    builder.notification(fcm_notif.finalize());
    let resp = fcm_client.send(builder.finalize()).await?;
    match resp {
        fcm::FcmResponse {
            failure: Some(f), ..
        } if f > 0 => {
            println!("failed to send notifs to {} ids", f);
        }
        fcm::FcmResponse {
            success: Some(f), ..
        } if f > 0 => {
            println!("successfully sent notifs to {} ids", f);
        }
        _ => {}
    }
    update_notification_timestamp(db, sub).await?;
    Ok(())
}

fn make_notification(centres: &[&getvacc::Center], sub: &Sub) -> String {
    format!(
        "{} slots open for {} age. Please check cowin website.",
        centres.len(),
        sub.age_limit
    )
}

fn get_date() -> String {
    let now = Local::today();
    now.format("%d-%m-%Y").to_string()
}

async fn update_notification_timestamp(db: &Pool<Sqlite>, sub: &Sub) -> Result<(), sqlx::Error> {
    let mut conn = db.acquire().await?;
    sqlx::query("UPDATE subs SET last_notification_at = ? WHERE pincode=? and age_limit=?")
        .bind(chrono::Utc::now().timestamp())
        .bind(&sub.pincode)
        .bind(&sub.age_limit)
        .execute(&mut conn)
        .await?;
    Ok(())
}
