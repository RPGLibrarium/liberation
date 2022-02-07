create table if not exists books (
  book_id               int auto_increment primary key,
  title_by_id           int not null,
  owner_member_by_id    int null,
  owner_guild_by_id     int null,
  -- owner_type            enum('guild', 'member') as (if(owner_guild_by_id is not null, 'guild', 'member')) STORED,
  quality               text not null,
  external_inventory_id int not null,
  unique key inventory_key (owner_member_by_id, owner_guild_by_id, external_inventory_id),
  foreign key (title_by_id) references titles (title_id),
  foreign key (owner_member_by_id) references accounts (account_id),
  foreign key (owner_guild_by_id) references guilds (guild_id),
  CHECK (owner_guild_by_id IS NOT NULL XOR owner_member_by_id IS NOT NULL)
) character set utf8mb4 collate utf8mb4_general_ci;
