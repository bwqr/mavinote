create function update_timestamp() returns trigger as $$
begin
    new.updated_at = now()::timestamptz(3);
    return new;
end;
$$ language plpgsql;

create type State as enum('Clean', 'Deleted');

create table users(
    id          serial primary key  not null,
    email       varchar(255)    not null unique,
    created_at  timestamp       not null default current_timestamp
);

create table pending_users(
    code        varchar(8)      not null,
    email       varchar(255)    not null,
    updated_at  timestamp       not null default current_timestamp,
    primary key (email)
);

create table pending_delete_users(
    user_id int  primary key    not null,
    code    varchar(8)      not null,
    updated_at  timestamp   not null default current_timestamp,
    constraint  fk_pending_delete_users_user_id foreign key (user_id) references users (id) on delete cascade on update no action
);

create table devices(
    id          serial  primary key not null,
    pubkey      varchar(64)     not null unique,
    password    varchar(128)    not null,
    created_at  timestamp       not null default current_timestamp
);

create table user_devices(
    user_id     int not null,
    device_id   int not null,
    created_at  timestamp   not null default current_timestamp,
    primary key (user_id, device_id),
    constraint  fk_user_devices_user_id foreign key (user_id) references users (id) on delete no action on update no action,
    constraint  fk_user_devices_device_id foreign key (device_id) references devices (id) on delete no action on update no action
);

create table pending_devices(
    user_id     int         not null,
    device_id   int         not null,
    updated_at  timestamp   not null default current_timestamp,
    primary key (user_id, device_id),
    constraint  fk_pending_devices_user_id foreign key (user_id) references users (id) on delete cascade on update no action,
    constraint  fk_pending_devices_device_id foreign key (device_id) references devices (id) on delete cascade on update no action
);

create table folders(
    id          serial  primary key not null,
    user_id     int         not null,
    state       State       not null default 'Clean',
    created_at  timestamp   not null default current_timestamp,
    constraint  fk_folders_user_id foreign key (user_id) references users (id) on delete no action on update no action
);

create table notes(
    id          serial  primary key not null,
    folder_id   int             not null,
    commit      int             not null default 1,
    state       State           not null default 'Clean',
    created_at  timestamp       not null default current_timestamp,
    updated_at  timestamp       not null default current_timestamp,
    constraint  fk_notes_folder_id foreign key (folder_id) references folders (id) on delete no action on update no action
);

create table device_folders(
    folder_id           int,
    sender_device_id    int not null,
    receiver_device_id  int not null,
    name                text    not null,
    primary key (folder_id, receiver_device_id),
    constraint  fk_device_folders_folder_id foreign key (folder_id) references folders (id) on delete cascade on update no action,
    constraint  fk_device_folders_sender_device_id foreign key (sender_device_id) references devices (id) on delete cascade on update no action,
    constraint  fk_device_folders_receiver_device_id foreign key (receiver_device_id) references devices (id) on delete cascade on update no action
);

create table device_notes(
    note_id             int     not null,
    sender_device_id    int     not null,
    receiver_device_id  int     not null,
    name                text    not null,
    text                text    not null,
    primary key (note_id, receiver_device_id),
    constraint  fk_device_notes_note_id foreign key (note_id) references notes (id) on delete cascade on update no action,
    constraint  fk_device_notes_sender_device_id foreign key (sender_device_id) references devices (id) on delete cascade on update no action,
    constraint  fk_device_notes_receiver_device_id foreign key (receiver_device_id) references devices (id) on delete cascade on update no action
);

create table note_requests(
    note_id     int not null,
    device_id   int not null,
    primary key (note_id, device_id),
    constraint  fk_note_requests_note_id foreign key (note_id) references notes (id) on delete cascade on update no action,
    constraint  fk_note_requests_device_id foreign key (device_id) references devices (id) on delete cascade on update no action
);

create table folder_requests(
    folder_id   int not null,
    device_id   int not null,
    primary key (folder_id, device_id),
    constraint  fk_folder_requests_folder_id foreign key (folder_id) references folders (id) on delete cascade on update no action,
    constraint  fk_folder_requests_device_id foreign key (device_id) references devices (id) on delete cascade on update no action
);

create trigger pending_users_updated_at
    before update
    on pending_users
    for each row
execute procedure update_timestamp();

create trigger pending_devices_updated_at
    before update
    on pending_devices
    for each row
execute procedure update_timestamp();

create trigger notes_updated_at
    before update
    on notes
    for each row
execute procedure update_timestamp();

create trigger pending_delete_users_updated_at
    before update
    on pending_delete_users
    for each row
execute procedure update_timestamp();
