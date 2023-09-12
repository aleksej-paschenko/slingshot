// @generated automatically by Diesel CLI.

diesel::table! {
    account_records (owner_id, alias) {
        owner_id -> Text,
        alias -> Text,
        wallet_id -> Text,
        wallet_json -> Text,
    }
}

diesel::table! {
    asset_records (owner_id, alias) {
        owner_id -> Text,
        alias -> Text,
        key_hex -> Text,
        flavor_hex -> Text,
    }
}

diesel::table! {
    block_records (height) {
        height -> Integer,
        header_json -> Text,
        txs_json -> Text,
        utxo_proofs_json -> Text,
        state_json -> Text,
    }
}

diesel::table! {
    user_records (id) {
        id -> Text,
        seed -> Text,
        info_json -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    account_records,
    asset_records,
    block_records,
    user_records,
);
