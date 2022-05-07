#include <stdbool.h>

void reax_init(const char * api_url, const char * storage_dir);
void reax_init_handler(void * ptr, void (*callback)(int wait_id, bool ok, const unsigned char *bytes, int bytes_len, void * ptr));

void reax_note_folders(int wait_id);
