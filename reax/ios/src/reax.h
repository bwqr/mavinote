#include <stdint.h>
#include <stdbool.h>

void reax_init(const char * api_url, const char * ws_url, const char * storage_dir);
void reax_init_handler(void * ptr, void (*callback)(int32_t id, const uint8_t *bytes, uintptr_t bytes_len, void * ptr));
void reax_abort(void * handle);

void * reax_account_accounts(int32_t stream_id);
void * reax_account_account(int32_t once_id, int32_t account_id);
void * reax_account_mavinote_account(int32_t once_id, int32_t account_id);
void * reax_account_devices(int32_t once_id, int32_t account_id);
void * reax_account_add_device(int32_t once_id, int32_t account_id, const char * fingerprint);
void * reax_account_delete_device(int32_t once_id, int32_t account_id, int32_t device_id);
void * reax_account_request_verification(int32_t once_id, const char * email);
void * reax_account_wait_verification(int32_t once_id, const char * token);
void * reax_account_add_account(int32_t once_id, const char * email);
void * reax_account_public_key(int32_t once_id);
void * reax_account_send_verification_code(int32_t once_id, const char * email);
void * reax_account_sign_up(int32_t once_id, const char * email, const char * code);
void * reax_account_remove_account(int32_t once_id, int32_t account_id);
void * reax_account_send_account_close_code(int32_t once_id, int32_t account_id);
void * reax_account_close_account(int32_t once_id, int32_t account_id, const char * code);
void * reax_account_listen_notifications(int32_t stream_id, int32_t account_id);

void reax_note_init();
void * reax_note_sync(int32_t once_id);
void * reax_note_folders(int32_t stream_id);
void * reax_note_folder(int32_t once_id, int32_t folder_id);
void * reax_note_create_folder(int32_t once_id, int32_t account_id, const char * name);
void * reax_note_delete_folder(int32_t once_id, int32_t folder_id);
void * reax_note_note_summaries(int32_t stream_id, int32_t folder_id);
void * reax_note_note(int32_t once_id, int32_t note_id);
void * reax_note_create_note(int32_t once_id, int32_t folder_id, const char * text);
void * reax_note_update_note(int32_t once_id, int32_t note_id, const char * text);
void * reax_note_delete_note(int32_t once_id, int32_t note_id);
