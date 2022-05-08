#include <stdbool.h>

void reax_init(const char * api_url, const char * storage_dir);
void reax_init_handler(void * ptr, void (*callback)(int wait_id, bool ok, const unsigned char *bytes, int bytes_len, void * ptr));

void reax_note_folders(int wait_id);
void reax_note_create_folder(int wait_id, const char * name);
void reax_note_note_summaries(int wait_id, int folder_id);
void reax_note_note(int wait_id, int note_id);
void reax_note_create_note(int wait_id, int folder_id);
void reax_note_update_note(int wait_id, int note_id, const char * text);
