use std::env;

use crate::setup_data::setup_data;

use self::model::*;
use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use load_database::read_onspd;
use schema::house_visits::dsl::*;
use schema::houses::dsl::*;

mod api;
mod load_database;
mod model;
mod schema;
mod setup_data;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn debug() {
    let connection = &mut establish_connection();

    setup_data(connection);

    let results = houses
        .limit(5)
        .select(House::as_select())
        .load(connection)
        .expect("Error Loading Houses.");

    for house in results {
        println!("{}, {}", house.house_number, house.street_id);
    }

    let results_2 = house_visits
        .limit(5)
        .select(HouseVisit::as_select())
        .load(connection)
        .expect("Error Loading Houses.");

    for visit in results_2 {
        println!(
            "{}, {}, {}",
            visit.timestamp,
            visit.id,
            visit.yp_voters.unwrap_or(-1)
        );
    }
}

fn main() {
    read_onspd();
}
