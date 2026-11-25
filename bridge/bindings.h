#pragma once

#ifdef __cplusplus
extern "C" {
#endif


int flask_which(const char *bin);


void flask_free_string(char *ptr);

#ifdef __cplusplus
}
#endif
