// @generated automatically by Diesel CLI.

diesel::table! {
    house_visits (id) {
        id -> Int4,
        house_id -> Int4,
        timestamp -> Timestamp,
        voter_count -> Nullable<Int4>,
        yp_voters -> Nullable<Int4>,
        lab_voters -> Nullable<Int4>,
        lib_voters -> Nullable<Int4>,
        con_voters -> Nullable<Int4>,
        ref_voters -> Nullable<Int4>,
        grn_voters -> Nullable<Int4>,
    }
}

diesel::table! {
    houses (id) {
        id -> Int4,
        street_id -> Int4,
        house_number -> Varchar,
    }
}

diesel::table! {
    streets (id) {
        id -> Int4,
        ward_id -> Int4,
        street_name -> Varchar,
    }
}

diesel::table! {
    wards (id) {
        id -> Int4,
        ward_name -> Varchar,
    }
}

diesel::joinable!(house_visits -> houses (house_id));
diesel::joinable!(houses -> streets (street_id));
diesel::joinable!(streets -> wards (ward_id));

diesel::allow_tables_to_appear_in_same_query!(house_visits, houses, streets, wards,);
