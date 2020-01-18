-- Cleanup for testing:
-- drop table if exists rentals, books, titles, rpg_systems, guilds, members;
-- DO NOT USE IN PRODUCTION!

-- Entity (aka members and guilds) tables
-- Have to be created created before books+rentals due to foreign keys
create table if not exists members (
  member_id                   int auto_increment primary key,
  external_id                 varchar(255) not null unique
) character set utf8mb4 collate utf8mb4_general_ci;

create table if not exists guilds (
  guild_id                    int auto_increment primary key,
  name                        varchar(255) not null unique,
  address                     text not null,
  contact_by_member_id        int not null,
  foreign key (contact_by_member_id) references members (member_id)
) character set utf8mb4 collate utf8mb4_general_ci;

-- "Book-related" tables

create table if not exists rpg_systems (
  rpg_system_id   int auto_increment primary key,
  name            varchar(255) unique not null,
  shortname       varchar(255) unique
) character set utf8mb4 collate utf8mb4_general_ci;

create table if not exists titles (
  title_id          int auto_increment primary key,
  name              varchar(255) not null unique,
  rpg_system_by_id  int not null,
  language          varchar(255) not null,
  publisher         varchar(255) not null,
  year              smallint not null,
  coverimage        text null,
  foreign key (rpg_system_by_id) references rpg_systems (rpg_system_id)
) character set utf8mb4 collate utf8mb4_general_ci;


create table if not exists books (
  book_id                 int auto_increment primary key,
  external_inventory_id int not null unique,
  title_by_id             int not null,
  owner_member_by_id      int null,
  owner_guild_by_id       int null,
  owner_type              enum('guild', 'member')
    as (if(owner_guild_by_id is not null, 'guild', 'member')) STORED,
  quality                 text not null,
  state                   enum('free', 'rented', 'destroyed', 'lost', 'reserved'),
  state_since             datetime,
  rentee_member_by_id      int null,
  rentee_guild_by_id       int null,
  rentee_type              enum('guild', 'member')
    as (if(rentee_guild_by_id is not null, 'guild', 'member')) STORED,
  foreign key (title_by_id) references titles (title_id),
  foreign key (owner_member_by_id) references members (member_id),
  foreign key (owner_guild_by_id) references guilds (guild_id),
  CHECK (owner_guild_by_id IS NOT NULL XOR owner_member_by_id IS NOT NULL),
  foreign key (rentee_member_by_id) references members (member_id),
  foreign key (rentee_guild_by_id) references guilds (guild_id),
  CHECK (rentee_guild_by_id IS NOT NULL XOR rentee_member_by_id IS NOT NULL)
) character set utf8mb4 collate utf8mb4_general_ci;

create table if not exists rental_audit (
  audit_id              int auto_increment primary key,
  book_by_id            int not null,
  before_state          varchar(255),
  after_state           varchar(255),
  before_rentee_guild_by_id int null,
  after_rentee_member_by_id int null,
  before_state_since    datetime,
  after_state_since     datetime
)

-- it might be better to remove the generated columns and replace them
-- with business logic in the backend code ...
