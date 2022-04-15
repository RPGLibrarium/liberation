alter table titles
  drop foreign key if exists titles_ibfk_1,
  add constraint titles_fk_rpg_system_by_id
    foreign key if not exists (rpg_system_by_id) references rpg_systems (rpg_system_id);

alter table librarians
  drop foreign key if exists librarians_ibfk_1,
  drop foreign key if exists librarians_ibfk_2,
  add constraint librarians_fk_guild_id
    foreign key if not exists (guild_id) references guilds (guild_id),
  add constraint librarians_fk_account_id
    foreign key if not exists (account_id) references accounts (account_id);

alter table books
  drop foreign key if exists books_ibfk_1,
  drop foreign key if exists books_ibfk_2,
  drop foreign key if exists books_ibfk_3,
  add constraint books_fk_title_by_id
    foreign key if not exists (title_by_id) references titles (title_id),
  add constraint books_fk_owner_member_by_id
    foreign key if not exists (owner_member_by_id) references accounts (account_id),
  add constraint books_fk_owner_guild_by_id
    foreign key if not exists (owner_guild_by_id) references guilds (guild_id);
