insert into members (member_id, external_id) values (999, 'REPLACE HERE');

insert into guilds (guild_id, name, address, contact_by_member_id) values (1, 'RPG Librarium Aachen', 'Schurzelter Str. 469, 52074 Aachen', 999);

insert into rpg_systems (rpg_system_id, shortname, name) (select a.value, if(description = '',null, name) as short_name, if(description = '',name,description) as name
  from tiki_tracker_item_fields a
  join tiki_categories b on a.value = b.categId
  where fieldId = 18
  group by categId);

insert into titles (title_id, rpg_system_by_id, name , language, publisher, year) (select i.itemId as title_id, s.value as rpg_system_by_id, if(n.value = 'Grundregelwerk', concat(i.itemId, n.value), n.value) as name, 'de' as language, '' as publisher, 0 as year
  from tiki_tracker_items i
  left join tiki_tracker_item_fields s on (i.itemId = s.itemId and s.fieldId = 18)
  left join tiki_tracker_item_fields n on (i.itemId = n.itemId and n.fieldId = 16)
  where i.trackerId = 3);

insert into books (book_id, title_by_id, owner_member_by_id, owner_guild_by_id, quality, external_inventory_id) (select i.itemId as book_id, t.value as title_by_id, null as owner_by_member_id, 1 as owner_by_guild_id, '' as quality, e.value as external_inventory_id 
  from tiki_tracker_items i
  left join tiki_tracker_item_fields t on (i.itemId = t.itemId and t.fieldId = 19)
  left join tiki_tracker_item_fields e on (i.itemId = e.itemId and e.fieldId = 20)
  where i.trackerId = 2);
