create table folders(
    id      integer primary key autoincrement,
    name    varchar(255) not null
);

create table notes(
    id          integer primary key autoincrement,
    folder_id   integer         not null,
    title       varchar(255)    not null,
    text        text            not null,
    foreign key(folder_id)  references  folders(id)
);

create table store(
    key     varchar(255)    not null    unique,
    value   text            not null
);
