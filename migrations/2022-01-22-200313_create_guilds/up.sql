create table if not exists guilds (
  guild_id                    int auto_increment primary key,
  name                        varchar(255) not null unique,
  address                     text not null,
  contact_by_member_id        int not null,
  foreign key (contact_by_member_id) references members (member_id)
) character set utf8mb4 collate utf8mb4_general_ci;