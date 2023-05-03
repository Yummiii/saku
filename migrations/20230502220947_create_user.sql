-- Add migration script here
alter table Users add column virtual boolean not null default false;
alter table Channels add column virtual_user bigint;

create table VirtualUsers (
    id bigint not null auto_increment,
    user_id bigint not null,
    virtual_user_id bigint not null,
    role tinyint unsigned not null,
    primary key (id),
    foreign key (user_id) references Users(id),
    foreign key (virtual_user_id) references Users(id)
);