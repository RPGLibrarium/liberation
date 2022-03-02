create table if not exists accounts (
  account_id                  int auto_increment primary key,
  active                      boolean not null,
  external_id                 varchar(255) not null unique,
  full_name                   varchar(255) not null unique,
  given_name                  varchar(255) not null,
  family_name                 varchar(255) not null,
  email                       varchar(255) not null
) character set utf8mb4 collate utf8mb4_general_ci;
