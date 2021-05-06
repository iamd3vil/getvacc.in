use serde::{Deserialize, Serialize};
use std::error::Error;
#[derive(Debug, Serialize, Deserialize)]
pub struct Centers {
    pub centers: Vec<Center>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Center {
    pub center_id: u64,
    pub name: String,
    pub state_name: String,
    pub district_name: String,
    pub block_name: String,
    pub pincode: u32,
    pub from: String,
    pub to: String,
    pub fee_type: String,
    pub sessions: Vec<Session>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub date: String,
    pub available_capacity: u32,
    pub min_age_limit: u32,
    pub vaccine: String,
    pub slots: Vec<String>,
}

pub async fn get_centers(pincode: &str, date: &str) -> Result<Centers, Box<dyn Error>> {
    let url = format!("https://cdn-api.co-vin.in/api/v2/appointment/sessions/public/calendarByPin?pincode={}&date={}", pincode, date);
    let resp = reqwest::get(url).await?.text().await?;
    let c: Centers = serde_json::from_str(&resp)?;
    Ok(c)
}
