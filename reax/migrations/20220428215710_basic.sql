create table folders(
    id          integer primary key autoincrement,
    remote_id   integer unique  default null,
    name        varchar(255)    not null,
    state       varchar(7)      not null    default 'Clean',
    check(state in ('Clean', 'Deleted'))
);

create table notes(
    id          integer primary key autoincrement,
    folder_id   integer         not null,
    remote_id   integer unique  default null,
    title       varchar(255)    default null,
    text        text            not null,
    commit_id   integer         not null,
    state       varchar(8)      not null,
    foreign key(folder_id)  references  folders(id) on delete cascade on update no action,
    check(state in ('Clean', 'Modified', 'Deleted'))
);

create table store(
    key     varchar(255)    not null    unique,
    value   text            not null
);
