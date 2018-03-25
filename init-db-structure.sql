create table if not exists rpg_systems (
  id          int auto_increment primary key,
  name        varchar(255) unique not null
) character set utf8mb4 collate utf8mb4_general_ci;

create table if not exists titles (
  id          int auto_increment primary key,
  name        varchar(255) not null unique,
  system      int not null,
  language    varchar(255) not null,
  publisher   varchar(255) not null,
  year        year not null,
  coverimage  text null,
  foreign key (system) references rpg_systems (id)
) character set utf8mb4 collate utf8mb4_general_ci;

create table if not exists books (
  id          int auto_increment primary key,
  title       int not null,
  owner_type  enum('guild', 'member') not null,
  owner_id    int not null,
  quality     text not null,
  foreign key (title) references titles (id)
) character set utf8mb4 collate utf8mb4_general_ci;

create table if not exists members (
  id          int auto_increment primary key,
  external_id varchar(255) not null unique
) character set utf8mb4 collate utf8mb4_general_ci;

create table if not exists guilds (
  id          int auto_increment primary key,
  name        varchar(255) not null unique,
  address     text not null,
  contact     int not null,
  foreign key (contact) references members (id)
) character set utf8mb4 collate utf8mb4_general_ci;

create table if not exists rentals (
  id          int auto_increment primary key,
  from_date   date not null,
  to_date     date not null,
  book        int not null,
  title       int not null,
  rentee_type enum('guild', 'member') not null,
  rentee_id   int not null,
  foreign key (book) references books (id),
  foreign key (title) references titles (id)
) character set utf8mb4 collate utf8mb4_general_ci;
