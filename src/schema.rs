table! {
    accounts (account_id) {
        account_id -> Integer,
        active -> Bool,
        external_id -> Varchar,
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
        contact_by_account_id -> Integer,
    }
}

table! {
    librarians (permission_id) {
        permission_id -> Integer,
        guild_id -> Integer,
        account_id -> Integer,
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
joinable!(books -> titles (title_by_id));
joinable!(guilds -> accounts (contact_by_account_id));
joinable!(librarians -> accounts (account_id));
joinable!(librarians -> guilds (guild_id));
joinable!(titles -> rpg_systems (rpg_system_by_id));

allow_tables_to_appear_in_same_query!(accounts, books, guilds, librarians, rpg_systems, titles,);
