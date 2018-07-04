---
title: Database Calls
layout: default
nav_link: Database Calls
nav_order: 210
nav_level: 2
lang: en
---

# Special DBCalls
- TEST GetTitlesBySystem (systemid)
- TEST GetRpgSystem(systemid) -> GetTitlesBySystem
- TEST GetBooksByTitle (titleid)
- TEST GetTitle (titleid) -> GetBooksByTitle
- TODO GetEntitiyByIdAndType
- TODO GetBookDetails -> GetTitleById, GetEntitiyByIdAndType
- TODO GetRentalsByBook(bookid)
- TODO isAvailable(bookid) -> GetRentalsbyBook

# Consideration:
1. Introduce Businesslogig level, sind De-/Serialization should be seperated from the logik.

2. Get Single, GetAll, Insert, Update for all Tables, use Traits?
  - [ ] GetSingleRpgSystem
  - [ ] GetAllRpgSystem
  - [ ] InsertRpgSystem
  - [ ] UpdateRpgSystem
  - [ ] GetSingleTitle
  - [ ] GetAllTitle
  - [ ] InsertTitle
  - [ ] UpdateTitle
  - [ ] GetSingleBook
  - [ ] GetAllBook
  - [ ] InsertBook
  - [ ] UpdateBook
  - [ ] GetSingleMember
  - [ ] GetAllMember
  - [ ] InsertMember
  - [ ] UpdateMember
  - [ ] GetSingleGuild
  - [ ] GetAllGuilds
  - [ ] InsertGuild
  - [ ] UpdateGuild

3. Special Usecases as function
  - Titles JOIN RpgSystems
  - Books JOIN (Titles JOIN RpgSystems) JOIN Entity
  -

  | API                         | Business                             | Database                                                          | Comment     |
|-----------------------------|--------------------------------------|-------------------------------------------------------------------|-------------|
| GET /rpgsystems             | get_rpg_systems()                    | get_rpg_systems()                                                 |             |
| GET /rpgsystem/{systemid}   | get_detailed_rpg_system(RpgSystemId) | get_rpg_system(RpgSystemId) get_titles_by_rpg_system(RpgSystemId) |  Rename API |
| POST /rpgsystems            | insert_rpg_system(dtos::system)      | insert_rpg_system(dmos::RpgSystem)                                |             |
| PUT /rpgsystem/{systemid}   | update_rpg_system(dtos::system)      | update_rpg_system(dmos::RpgSystem)                                | Rename API  |
| GET /titles                 | get_titles()                         | get_titles() get_rpg_system(RpgSystemId)                          | JOIN?       |
| GET /titles/{titleid}       | get_detailed_title(TitleId)          | get_rpg_system(RpgSystemId)                                       | JOIN?       |
| GET /titles/{titleid}/books | get_books_by_title(TitleId)          | get_books_by_title(TitleId)           
