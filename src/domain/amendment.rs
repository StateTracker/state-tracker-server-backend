use rbatis::rbdc::datetime::DateTime;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amendments {
    pub uid: String,
    pub examination_ref: String,
    pub tri_amendment: String,
    pub legislative_text_ref: String,
    pub delivery_date: Option<DateTime>,
    pub publication_date: Option<DateTime>,
    pub sort_date: Option<DateTime>,
    pub state: String,
    pub sub_state: Option<String>,
    pub representation: String,
    pub article99: bool,
}

impl fmt::Display for Amendments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.uid)
    }
}

crud!(Amendments{});
impl_select_page!(Amendments{select_all_paginated(order_by: &str, sort_order: &str) => "
    if !sql.contains('count'):
        `order by ${order_by} ${sort_order}, uid offset ${page_size} * ${page_no} rows fetch next ${page_size} rows only --`"});
impl_select!(Amendments{select_by_uid(uid:String) -> Option => "`where uid = #{uid}`"});