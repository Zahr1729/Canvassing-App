use crate::model::*;
use crate::schema::*;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::PgConnection;
use diesel::prelude::*;

pub fn setup_data(conn: &mut PgConnection) {
    let ward = new_ward(conn, "DEMO WARD");
    let street = new_street(conn, "DEMO STREET", &ward.id);
    let house = new_house(conn, "10A", &street.id);
    let _house_visit = new_house_visit(
        conn,
        &house.id,
        Utc::now().naive_utc(),
        Some(3),
        Some(1),
        None,
        None,
        None,
        None,
        None,
    );
}

pub fn new_ward(conn: &mut PgConnection, name: &str) -> Ward {
    diesel::insert_into(wards::table)
        .values((wards::id.eq(0), wards::ward_name.eq(name)))
        .returning(Ward::as_returning())
        .get_result(conn)
        .expect(&("Error Saving Ward ".to_owned() + name))
}

pub fn new_street(conn: &mut PgConnection, name: &str, ward_id: &i32) -> Street {
    diesel::insert_into(streets::table)
        .values((
            streets::id.eq(0),
            streets::street_name.eq(name),
            streets::ward_id.eq(ward_id),
        ))
        .returning(Street::as_returning())
        .get_result(conn)
        .expect(&("Error Saving Street ".to_owned() + name))
}

pub fn new_house(conn: &mut PgConnection, house_number: &str, street_id: &i32) -> House {
    diesel::insert_into(houses::table)
        .values((
            houses::id.eq(0),
            houses::house_number.eq(house_number),
            houses::street_id.eq(street_id),
        ))
        .returning(House::as_returning())
        .get_result(conn)
        .expect(&("Error Saving House ".to_owned() + house_number))
}

pub fn new_house_visit(
    conn: &mut PgConnection,
    house_id: &i32,
    timestamp: NaiveDateTime,
    voter_count: Option<i32>,
    con_voters: Option<i32>,
    grn_voters: Option<i32>,
    lab_voters: Option<i32>,
    lib_voters: Option<i32>,
    ref_voters: Option<i32>,
    yp_voters: Option<i32>,
) -> HouseVisit {
    diesel::insert_into(house_visits::table)
        .values((
            house_visits::id.eq(0),
            house_visits::house_id.eq(house_id),
            house_visits::timestamp.eq(timestamp),
            house_visits::voter_count.eq(voter_count),
            house_visits::con_voters.eq(con_voters),
            house_visits::grn_voters.eq(grn_voters),
            house_visits::lab_voters.eq(lab_voters),
            house_visits::lib_voters.eq(lib_voters),
            house_visits::ref_voters.eq(ref_voters),
            house_visits::yp_voters.eq(yp_voters),
        ))
        .returning(HouseVisit::as_returning())
        .get_result(conn)
        .expect("Error Saving Visit")
}
