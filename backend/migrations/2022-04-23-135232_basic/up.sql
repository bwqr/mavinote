CREATE FUNCTION update_timestamp() RETURNS TRIGGER
    LANGUAGE plpgsql
AS
$$
BEGIN
    NEW.updated_at = now()::timestamptz(3);
    RETURN NEW;
END;
$$;

create table users(
    id      serial primary key  not null,
    name    varchar(255)    not null,
    email   varchar(255)    not null unique,
    password    varchar(88) not null,
    created_at  timestamp   not null default current_timestamp
);

create table folders(
    id      serial  primary key not null,
    user_id int             not null,
    name    varchar(255)    not null,
    created_at  timestamp   not null default current_timestamp,
    constraint  fk_folders_user_id foreign key (user_id) references users (id) on delete no action on update no action
);

create table notes(
    id      serial  primary key not null,
    folder_id   int             not null,
    title       varchar(255)    not null,
    text        text            not null,
    created_at  timestamp   not null default current_timestamp,
    updated_at  timestamp   not null default current_timestamp,
    constraint  fk_folders_user_id foreign key (folder_id) references folders (id) on delete cascade on update no action
);

create trigger notes_updated_at
    before update
    on notes
    for each row
    execute procedure update_timestamp();
