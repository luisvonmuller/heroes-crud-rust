table! {
    heroes (id) {
        id -> Int4,
        fantasy_name -> Varchar,
        real_name -> Nullable<Varchar>,
        spotted_photo -> Text,
        strength_level -> Int4,
    }
}
