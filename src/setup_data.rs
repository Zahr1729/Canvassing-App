use crate::model::*;
use crate::schema::*;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::PgConnection;
use diesel::prelude::*;

pub fn setup_data(conn: &mut PgConnection) {
    let ward = new_ward(
        conn,
        &NewWard {
            ward_name: "DEMO WARD".to_string(),
        },
    );
    let street = new_street(
        conn,
        &NewStreet {
            ward_id: ward.id,
            street_name: "DEMO STREET".to_string(),
        },
    );
    let house = new_house(
        conn,
        &NewHouse {
            street_id: street.id,
            house_number: "44".to_string(),
        },
    );
    let _house_visit = new_house_visit(
        conn,
        &NewHouseVisit {
            house_id: house.id,
            timestamp: Utc::now().naive_utc(),
            voter_count: Some(3),
            yp_voters: Some(1),
            lab_voters: None,
            lib_voters: None,
            con_voters: None,
            ref_voters: None,
            grn_voters: None,
        },
    );
}

pub fn new_ward(conn: &mut PgConnection, new_ward: &NewWard) -> Ward {
    diesel::insert_into(wards::table)
        .values(new_ward)
        .returning(Ward::as_returning())
        .get_result(conn)
        .expect(&("Error Saving Ward ".to_owned() + new_ward.ward_name.as_str()))
}

pub fn new_wards(conn: &mut PgConnection, new_wards: &Vec<NewWard>) -> Vec<Ward> {
    diesel::insert_into(wards::table)
        .values(new_wards)
        .get_results(conn)
        .expect("Error Saving Wards.")
}

pub fn new_street(conn: &mut PgConnection, new_street: &NewStreet) -> Street {
    diesel::insert_into(streets::table)
        .values(new_street)
        .returning(Street::as_returning())
        .get_result(conn)
        .expect(&("Error Saving Street ".to_owned() + new_street.street_name.as_str()))
}

pub fn new_streets(conn: &mut PgConnection, new_streets: &Vec<NewStreet>) -> Vec<Street> {
    diesel::insert_into(streets::table)
        .values(new_streets)
        .get_results(conn)
        .expect("Error Saving Streets.")
}

pub fn new_house(conn: &mut PgConnection, new_house: &NewHouse) -> House {
    diesel::insert_into(houses::table)
        .values(new_house)
        .returning(House::as_returning())
        .get_result(conn)
        .expect(&("Error Saving House ".to_owned() + new_house.house_number.as_str()))
}

pub fn new_houses(conn: &mut PgConnection, new_houses: &Vec<NewHouse>) -> Vec<House> {
    diesel::insert_into(houses::table)
        .values(new_houses)
        .get_results(conn)
        .expect("Error Saving House.")
}

pub fn new_house_visit(conn: &mut PgConnection, new_house_visit: &NewHouseVisit) -> HouseVisit {
    diesel::insert_into(house_visits::table)
        .values(new_house_visit)
        .returning(HouseVisit::as_returning())
        .get_result(conn)
        .expect("Error Saving Visit")
}

pub fn new_house_visits(
    conn: &mut PgConnection,
    new_house_visits: &Vec<NewHouseVisit>,
) -> Vec<HouseVisit> {
    diesel::insert_into(house_visits::table)
        .values(new_house_visits)
        .get_results(conn)
        .expect("Error Saving Visits.")
}
