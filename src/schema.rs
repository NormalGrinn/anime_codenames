// @generated automatically by Diesel CLI.

diesel::table! {
    games (channel_id) {
        channel_id -> BigInt,
        players -> Text,
        board -> Text,
        game -> Text,
    }
}
