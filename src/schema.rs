// @generated automatically by Diesel CLI.

diesel::table! {
    games (channel_id) {
        channel_id -> BigInt,
        players -> Text,
        board -> Text,
        game -> Text,
    }
}

diesel::table! {
    clue_info (id) {
        id -> BigInt,
        channel_id -> BigInt,
        clue_type -> Text,
        clue_body -> Text,
    }
}