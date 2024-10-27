// @generated automatically by Diesel CLI.

diesel::table! {
    clue_info (clue_id) {
        clue_id -> Integer,
        channel_id -> BigInt,
        clue_type -> Text,
        clue_body -> Text,
    }
}

diesel::table! {
    games (channel_id) {
        channel_id -> BigInt,
        players -> Text,
        board -> Text,
        game -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    clue_info,
    games,
);
