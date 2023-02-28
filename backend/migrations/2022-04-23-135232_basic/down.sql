drop trigger pending_devices_updated_at on pending_devices;
drop trigger pending_users_updated_at on pending_users;
drop trigger notes_updated_at on notes;

drop table note_requests;

drop table folder_requests;

drop table device_notes;

drop table device_folders;

drop table notes;

drop table folders;

drop table pending_devices;

drop table devices;

drop table pending_delete_users;

drop table pending_users;

drop table users;

drop type State;

drop function update_timestamp;
