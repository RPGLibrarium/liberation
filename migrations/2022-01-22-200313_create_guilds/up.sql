create table if not exists guilds (
  guild_id                    int auto_increment primary key,
  external_id                 varchar(255) not null unique,
  name                        varchar(255) not null unique,
  address                     text not null,
  contact_by_account_id       int not null,
  foreign key (contact_by_account_id) references accounts (account_id)
) character set utf8mb4 collate utf8mb4_general_ci;
