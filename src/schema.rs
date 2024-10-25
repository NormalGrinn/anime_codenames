// @generated automatically by Diesel CLI.

diesel::table! {
    games (channel_id) {
        channel_id -> Integer,
        players -> Nullable<Text>,
        board -> Nullable<Text>,
        game -> Nullable<Text>,
    }
}
