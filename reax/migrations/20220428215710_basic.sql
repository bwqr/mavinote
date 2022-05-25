create table folders(
    id      integer primary key autoincrement,
    name    varchar(255)    not null,
    kind    varchar(10)     not null check(kind in ('Local', 'Remote')),
    updated_at  integer     not null default current_timestamp
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
