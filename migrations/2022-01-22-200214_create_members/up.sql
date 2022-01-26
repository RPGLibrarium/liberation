create table if not exists members (
  member_id                   int auto_increment primary key,
  external_id                 varchar(255) not null unique
) character set utf8mb4 collate utf8mb4_general_ci;
