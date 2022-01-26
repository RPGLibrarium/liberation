create table if not exists rpg_systems (
  rpg_system_id   int auto_increment primary key,
  name            varchar(255) unique not null,
  shortname       varchar(255) unique
) character set utf8mb4 collate utf8mb4_general_ci;
