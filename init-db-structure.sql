-- Cleanup for testing:
-- drop table if exists rentals, books, titles, rpg_systems, guilds, members;
-- DO NOT USE IN PRODUCTION!

-- Entity (aka members and guilds) tables
-- Have to be created created before books+rentals due to foreign keys

create table if not exists members (
  id              int auto_increment primary key,
  external_id     varchar(255) not null unique
) character set utf8mb4 collate utf8mb4_general_ci;

create table if not exists guilds (
  id              int auto_increment primary key,
  name            varchar(255) not null unique,
  address         text not null,
  contact         int not null,
  foreign key (contact) references members (id)
) character set utf8mb4 collate utf8mb4_general_ci;


-- "Book-related" tables

create table if not exists rpg_systems (
  id              int auto_increment primary key,
  name            varchar(255) unique not null
) character set utf8mb4 collate utf8mb4_general_ci;

create table if not exists titles (
  id              int auto_increment primary key,
  name            varchar(255) not null unique,
  system          int not null,
  language        varchar(255) not null,
  publisher       varchar(255) not null,
  year            year not null,
  coverimage      text null,
  foreign key (system) references rpg_systems (id)
) character set utf8mb4 collate utf8mb4_general_ci;

create table if not exists books (
  id              int auto_increment primary key,
  title           int not null,
  owner_guild     int null,
  owner_member    int null,
  owner_type      enum('guild', 'member')
      as (if(owner_guild is not null, 'guild', 'member')) STORED,
  quality         text not null,
  foreign key (title) references titles (id),
  foreign key (owner_guild) references guilds (id),
  foreign key (owner_member) references members (id),
  CHECK (owner_guild IS NOT NULL XOR owner_member IS NOT NULL)
) character set utf8mb4 collate utf8mb4_general_ci;

create table if not exists rentals (
  id              int auto_increment primary key,
  from_date       date not null,
  to_date         date not null,
  book            int not null,
  title           int not null,
  rentee_guild    int null,
  rentee_member   int null,
  rentee_type      enum('guild', 'member')
      as (if(rentee_guild is not null, 'guild', 'member')) STORED,
  quality         text not null,
  foreign key (book) references books (id),
  foreign key (title) references titles (id),
  foreign key (rentee_guild) references guilds (id),
  foreign key (rentee_member) references members (id),
  CHECK (rentee_guild IS NOT NULL XOR rentee_member IS NOT NULL)
) character set utf8mb4 collate utf8mb4_general_ci;

-- it might be better to remove the generated columns and replace them
-- with business logic in the backend code ...
