// @generated automatically by Diesel CLI.

diesel::table! {
    games (channel_id) {
        channel_id -> BigInt,
        players -> Nullable<Text>,
        board -> Nullable<Text>,
        game -> Nullable<Text>,
    }
}
