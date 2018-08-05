---
title: Data Model
layout: default
nav_link: Data Model
nav_order: 200
nav_level: 1
lang: en
---

| Tables | Description |
|-------------|--------------------------------------------|
| [rpg_systems](#rpg_systems) | All the RpgSystems |
| [titles](#titles) | Titles with RpgSystems |
| [books](#books) | Books with Titles |
| [members](#members) | Mapping between memberIds and keycload Ids |
| [guilds](#guilds) | All guilds |
| [rentals](#rentals) | All rentals |

### rpg_systems

| Column | Type | |
|--------|--------------------------------|--|
| rpg_system_id | int auto_increment primary key |
| name | varchar(255) not null unique |

### titles

| Column | Type | References |
|------------|--------------------------------|---------------|
| title_id | int auto_increment primary key |  |
| name | varchar(255) not null unique |  |
| rpg_system_by_id | int not null | rpg_system.rpg_system_id |
| language | varchar |  |
| publisher | varchar |  |
| year | smallint |  |
| coverimage | text |  |

### books

| Column | Type | References |
|------------|--------------------------------|---------------------|
| book_id | int auto_increment primary key |  |
| title_by_id | int | titles.title_id |
| owner_member_by_id | int null | member.member_id |
| owner_guild_by_id | int null | guild.guild_id |
| owner_type | ENUM(member, guild) |  |
| quality | text |  |

### members

| Column | Type | References |
|-------------|--------------------------------|---------------------|
| member_id | int auto_increment primary key |  |
| external_id | varchar unique |  |

### guilds

| Column | Type | References |
|------------|--------------------------------|------------|
| guild_id | int auto_increment primary key |  |
| name | varchar unique |  |
| address | text |  |
| contact_by_member_id | int  | member.member_id |

### rentals

| Column | Type | References |
|-------------|--------------------------------|---------------------|
| rental_id | int auto_increment primary key |  |
| from_date | date |  |
| to_date | date |  |
| book_by_id | int  | books.book_id |
| rentee_member_by_id | int null | member.member_id |
| rentee_guild_by_id | int null | guild.guild_id |
| rentee_type | ENUM('member', 'guild') |  |
