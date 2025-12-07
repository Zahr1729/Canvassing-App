// Bespoke Created.

diesel::table! {
    wards (id) {
        id -> Serial,
        ward_name -> VarChar,
    }
}

diesel::table! {
    streets (id) {
        id -> Serial,
        ward_id -> Integer,
        street_name -> VarChar,
    }
}

diesel::table! {
    houses (id) {
        id -> Serial,
        street_id -> Integer,
        house_number -> VarChar,
    }
}

diesel::table! {
    house_visits (id) {
        id -> Serial,
        house_id -> Integer,
        timestamp -> Timestamp,
        voter_count -> Nullable<Integer>,
        yp_voters -> Nullable<Integer>,
        lab_voters -> Nullable<Integer>,
        lib_voters -> Nullable<Integer>,
        con_voters -> Nullable<Integer>,
        ref_voters -> Nullable<Integer>,
        grn_voters -> Nullable<Integer>,
    }
}

diesel::joinable!(house_visits -> houses (house_id));
diesel::joinable!(houses -> streets (street_id));
diesel::joinable!(streets -> wards (ward_id));

diesel::allow_tables_to_appear_in_same_query!(house_visits, houses, streets, wards,);
