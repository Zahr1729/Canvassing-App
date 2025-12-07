use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{self, Selectable};

use crate::schema::*;

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(belongs_to(House))]
#[diesel(table_name = house_visits)]
pub struct HouseVisit {
    pub id: i32,
    pub house_id: i32,
    pub timestamp: NaiveDateTime,
    pub voter_count: Option<i32>,
    pub yp_voters: Option<i32>,
    pub lab_voters: Option<i32>,
    pub lib_voters: Option<i32>,
    pub con_voters: Option<i32>,
    pub ref_voters: Option<i32>,
    pub grn_voters: Option<i32>,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(belongs_to(House))]
#[diesel(table_name = house_visits)]
pub struct NewHouseVisit {
    pub house_id: i32,
    pub timestamp: NaiveDateTime,
    pub voter_count: Option<i32>,
    pub yp_voters: Option<i32>,
    pub lab_voters: Option<i32>,
    pub lib_voters: Option<i32>,
    pub con_voters: Option<i32>,
    pub ref_voters: Option<i32>,
    pub grn_voters: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(belongs_to(Street))]
#[diesel(table_name = houses)]
pub struct House {
    pub id: i32,
    pub street_id: i32,
    pub house_number: String,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(belongs_to(Street))]
#[diesel(table_name = houses)]
pub struct NewHouse {
    pub street_id: i32,
    pub house_number: String,
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(belongs_to(Ward))]
#[diesel(table_name = streets)]
pub struct Street {
    pub id: i32,
    pub ward_id: i32,
    pub street_name: String,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(belongs_to(Ward))]
#[diesel(table_name = streets)]
pub struct NewStreet {
    pub ward_id: i32,
    pub street_name: String,
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = wards)]
pub struct Ward {
    pub id: i32,
    pub ward_name: String,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = wards)]
pub struct NewWard {
    pub ward_name: String,
}
