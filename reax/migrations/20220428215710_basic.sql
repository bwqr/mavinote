create table accounts(
    id      integer primary key autoincrement,
    name    text        not null    unique,
    kind    varchar(9)  not null,
    data    text    default null,
    check(kind in ('Mavinote', 'Local'))
);

insert into accounts (name, kind) values ('Local', 'Local');

create table devices(
    id  integer,
    account_id  integer     not null,
    pubkey      varchar(64) not null,
    created_at  text        not null,
    foreign key(account_id) references accounts(id) on delete cascade on update no action,
    unique(id, account_id)
);

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
    remote_id   integer         default null,
    'commit'    integer         not null,
    name        varchar(255)    not null,
    text        text            not null,
    state       varchar(8)      not null,
    foreign key(folder_id) references folders(id) on delete cascade on update no action,
    unique(folder_id, remote_id),
    check(state in ('Clean', 'Modified', 'Deleted'))
);

create table store(
    key     varchar(128)    not null    unique,
    value   text            not null
);
