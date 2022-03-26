alter table guilds
  add column if not exists contact_by_account_id int not null,
  add foreign key (contact_by_account_id) references accounts (account_id);
