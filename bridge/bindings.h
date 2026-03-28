#pragma once

#ifdef __cplusplus
extern "C" {
#endif

/* Returns 1 if `bin` is found in PATH, 0 otherwise. */
int flask_which(const char *bin);

/* Frees a string allocated on the Rust side. */
void flask_free_string(char *ptr);

#ifdef __cplusplus
}
#endif
