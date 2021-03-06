create table accounts(
    id      integer primary key autoincrement,
    name    text        not null    unique,
    kind    varchar(9)  not null,
    check(kind in ('Mavinote', 'Local'))
);

insert into accounts (name, kind) values ('Local', 'Local');

create table folders(
    id          integer primary key autoincrement,
    account_id  integer         not null,
    remote_id   integer default null,
    name        varchar(255)    not null,
    state       varchar(7)      not null    default 'Clean',
    foreign key(account_id) references accounts(id) on delete cascade on update no action,
    unique(account_id, remote_id),
    check(state in ('Clean', 'Deleted'))
);

create table notes(
    id          integer primary key autoincrement,
    folder_id   integer         not null,
    remote_id   integer default null,
    title       varchar(255)    default null,
    text        text            not null,
    commit_id   integer         not null,
    state       varchar(8)      not null,
    foreign key(folder_id) references folders(id) on delete cascade on update no action,
    unique(folder_id, remote_id),
    check(state in ('Clean', 'Modified', 'Deleted'))
);

create table store(
    key     varchar(255)    not null    unique,
    value   text            not null
);
