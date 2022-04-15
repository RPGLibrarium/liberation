create table if not exists inventories (
  inventory_id          int auto_increment primary key,
  owner_member_by_id    int null unique,
  owner_guild_by_id     int null,
  description           text not null,
  foreign key (owner_member_by_id) references accounts (account_id),
  foreign key (owner_guild_by_id) references guilds (guild_id),
  constraint xor_owner check (owner_guild_by_id is not null xor owner_member_by_id is not null)
) character set utf8mb4 collate utf8mb4_general_ci;

insert ignore into inventories (owner_member_by_id, description)
select account_id, "member inventory" from accounts;

insert ignore into inventories (owner_guild_by_id, description)
select guild_id, "default inventory" from guilds;

alter table books
  add column if not exists inventory_by_id int after owner_guild_by_id,
  add constraint fk_inventory foreign key (inventory_by_id) references inventories (inventory_id);

update books set inventory_by_id = (
    select inventory_id from inventories
    where (inventories.owner_member_by_id = books.owner_member_by_id
      or inventories.owner_guild_by_id = books.owner_guild_by_id)
    -- fetch first TODO: help needed
  );

alter table books
  modify column inventory_by_id int not null;
