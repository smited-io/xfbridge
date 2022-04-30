use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Debug)]
pub struct StatsResponse {
    pub totals: Totals,
    pub latest_user: LatestUser,
    pub online: Online,
}

#[derive(Deserialize, Debug)]
pub struct Totals {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub threads: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub messages: u64,
    pub users: u64,
}

#[derive(Deserialize, Debug)]
pub struct LatestUser {
    pub user_id: u64,
    pub username: String,
    pub register_date: u64,
}

#[derive(Deserialize, Debug)]
pub struct Online {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub members: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub guests: u64,
}
