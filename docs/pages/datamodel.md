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
| id | int auto_increment primary key |
| name | varchar(255) not null unique |

### titles

| Column | Type | References |
|------------|--------------------------------|---------------|
| id | int auto_increment primary key |  |
| name | varchar(255) not null unique |  |
| system | int not null | rpg_system.id |
| language | varchar |  |
| publisher | varchar |  |
| year | year |  |
| coverimage | text |  |

### books

| Column | Type | References |
|------------|--------------------------------|---------------------|
| id | int auto_increment primary key |  |
| title | int | titles |
| owner_type | ENUM(member, guild) |  |
| owner_id | int  | member.id, guild.id |
| quality | text |  |

### members

| Column | Type | References |
|-------------|--------------------------------|---------------------|
| id | int auto_increment primary key |  |
| external_id | varchar unique |  |

### guilds

| Column | Type | References |
|------------|--------------------------------|------------|
| id | int auto_increment primary key |  |
| name | varchar unique |  |
| address | text |  |
| contact | int  | member.id |

### rentals

| Column | Type | References |
|-------------|--------------------------------|---------------------|
| id | int auto_increment primary key |  |
| from | date |  |
| to | date |  |
| book | int  | books.id |
| rentee_type | ENUM(member, guild) |  |
| rentee_id | int | member.id, guild.id |
