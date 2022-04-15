table! {
    accounts (account_id) {
        account_id -> Integer,
        active -> Bool,
        external_id -> Varchar,
        username -> Varchar,
        full_name -> Varchar,
        given_name -> Varchar,
        family_name -> Varchar,
        email -> Varchar,
    }
}

table! {
    books (book_id) {
        book_id -> Integer,
        title_by_id -> Integer,
        owner_member_by_id -> Nullable<Integer>,
        owner_guild_by_id -> Nullable<Integer>,
        inventory_by_id -> Integer,
        quality -> Text,
        external_inventory_id -> Integer,
    }
}

table! {
    guilds (guild_id) {
        guild_id -> Integer,
        external_id -> Varchar,
        name -> Varchar,
        address -> Text,
    }
}

table! {
    inventories (inventory_id) {
        inventory_id -> Integer,
        owner_member_by_id -> Nullable<Integer>,
        owner_guild_by_id -> Nullable<Integer>,
        description -> Text,
    }
}

table! {
    rpg_systems (rpg_system_id) {
        rpg_system_id -> Integer,
        name -> Varchar,
        shortname -> Nullable<Varchar>,
    }
}

table! {
    titles (title_id) {
        title_id -> Integer,
        name -> Varchar,
        rpg_system_by_id -> Integer,
        language -> Varchar,
        publisher -> Varchar,
        year -> Smallint,
        coverimage -> Nullable<Text>,
    }
}

joinable!(books -> accounts (owner_member_by_id));
joinable!(books -> guilds (owner_guild_by_id));
joinable!(books -> inventories (inventory_by_id));
joinable!(books -> titles (title_by_id));
joinable!(inventories -> accounts (owner_member_by_id));
joinable!(inventories -> guilds (owner_guild_by_id));
joinable!(titles -> rpg_systems (rpg_system_by_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    books,
    guilds,
    inventories,
    rpg_systems,
    titles,
);
