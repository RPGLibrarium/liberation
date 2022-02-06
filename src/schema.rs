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
        external_guild_name -> Varchar,
        name -> Varchar,
        address -> Text,
        contact_by_member_id -> Integer,
    }
}

table! {
    members (member_id) {
        member_id -> Integer,
        external_id -> Varchar,
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

joinable!(books -> guilds (owner_guild_by_id));
joinable!(books -> members (owner_member_by_id));
joinable!(books -> titles (title_by_id));
joinable!(guilds -> members (contact_by_member_id));
joinable!(titles -> rpg_systems (rpg_system_by_id));

allow_tables_to_appear_in_same_query!(
    books,
    guilds,
    members,
    rpg_systems,
    titles,
);
