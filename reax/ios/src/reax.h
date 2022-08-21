#include <stdbool.h>

void reax_init(const char * api_url, const char * notify_url, const char * storage_dir);
void reax_init_handler(void * ptr, void (*callback)(int id, bool is_stream, const unsigned char *bytes, int bytes_len, void * ptr));
void reax_abort(void * handle);

void * reax_note_accounts(int stream_id);
void * reax_note_account(int once_id, int account_id);
void * reax_note_mavinote_account(int once_id, int account_id);
void * reax_note_add_account(int once_id, const char * name, const char * email, const char * password, bool create_account);
void * reax_note_delete_account(int once_id, int account_id);
void * reax_note_sync(int once_id);
void * reax_note_folders(int stream_id);
void * reax_note_folder(int once_id, int folder_id);
void * reax_note_create_folder(int once_id, int account_id, const char * name);
void * reax_note_delete_folder(int once_id, int folder_id);
void * reax_note_note_summaries(int stream_id, int folder_id);
void * reax_note_note(int once_id, int note_id);
void * reax_note_create_note(int once_id, int folder_id, const char * text);
void * reax_note_update_note(int once_id, int note_id, const char * text);
void * reax_note_delete_note(int once_id, int note_id);

void * reax_auth_login(int once_id, const char * email, const char * password);
