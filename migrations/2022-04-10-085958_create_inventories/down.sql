alter table books
  drop constraint fk_inventory,
  drop column inventory_by_id;

drop table inventories;
