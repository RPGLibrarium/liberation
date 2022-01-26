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

