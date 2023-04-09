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
  primary key (Id)
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
  foreign key (user) references Users(id),
  foreign key (channel) references Channels(id),
  primary key (id)
);
create index IX_Contexts_Channel on Contexts(channel);
