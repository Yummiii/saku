-- Add migration script here
alter table UsageLog add column multiplier float;

alter table UsageLog add column model tinyint unsigned not null default 1;
alter table Channels add column model tinyint unsigned not null default 1;