create table users (
    id  serial primary key  not null,
    name    varchar(255)    not null,
    email   varchar(255)    not null unique,
    password    varchar(88) not null,
    created_at  timestamp   not null default current_timestamp
);
