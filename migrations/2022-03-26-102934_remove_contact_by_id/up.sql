alter table guilds
  drop constraint if exists guilds_ibfk_1,
  drop column contact_by_account_id;
