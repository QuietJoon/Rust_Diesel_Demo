use crate::schema::*;
use diesel::{Queryable,Insertable};

#[derive(Queryable,Insertable)]
#[diesel(table_name = core_infos)]
pub struct CoreInfo {
    pub id: i32,
    pub seed: i32,
    pub session_num: i32,
    pub is_running: bool,
    pub body: String,
}

#[derive(Queryable,Insertable)]
#[diesel(table_name = gen_perf_infos)]
pub struct GenPerfInfo {
    pub target: i32,
    pub target_name: String,
    pub available: bool,
    pub body: String,
}
