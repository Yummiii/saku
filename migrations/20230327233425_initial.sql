-- Add migration script here
create table Users (
  id bigint not null auto_increment,
  discord_id bigint not null,
  name varchar(255) not null,
  state tinyint unsigned not null,
  primary key (id)
);
create index IX_Users_DiscordId on Users(discord_id);

create table Channels (
  id bigint not null auto_increment,
  discord_id bigint not null,
  state tinyint unsigned not null,
  ccid varchar(24) not null,
  primary key (id)
);
create index IX_Channels_DiscordId on Channels(discord_id);

create table Contexts (
  id bigint not null auto_increment,
  role varchar(255) not null,
  message text not null,
  active boolean not null,
  created_at bigint not null,
  channel bigint not null,
  user bigint,
  cid varchar(24) not null,
  foreign key (user) references Users(id),
  foreign key (channel) references Channels(id),
  primary key (id)
);
create index IX_Contexts_Channel on Contexts(channel);

create table UsageLog (
  id bigint not null auto_increment,
  created_at bigint not null,
  prompt_tokens int not null,
  completion_tokens int not null,
  cid varchar(24) not null,
  user bigint not null,
  foreign key (user) references Users(id),
  primary key (id)
);