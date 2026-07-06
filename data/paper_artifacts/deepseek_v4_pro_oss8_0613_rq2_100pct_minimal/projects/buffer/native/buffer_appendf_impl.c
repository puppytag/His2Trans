/* Implementation of buffer_appendf as a C shim to handle variadic arguments. */
#include <stdlib.h>
#include <stdarg.h>
#include <stdio.h>
#include <stddef.h>  /* for size_t */

/* Opaque forward declaration; do NOT include buffer.h */
typedef struct buffer_t buffer_t;

/* Rust-provided shim functions */
extern size_t buffer_shim_length(buffer_t *self);
extern int buffer_shim_resize(buffer_t *self, size_t n);
extern char *buffer_shim_data_at(buffer_t *self, size_t off);

int buffer_appendf(buffer_t *self, const char *format, ...) {
    va_list ap;
    va_list tmpa;
    char *dst = NULL;
    int length = 0;
    int required = 0;
    int bytes = 0;

    va_start(ap, format);

    length = buffer_shim_length(self);

    va_copy(tmpa, ap);
    required = vsnprintf(NULL, 0, format, tmpa);
    va_end(tmpa);
    if (-1 == buffer_shim_resize(self, length + required)) {
        va_end(ap);
        return -1;
    }

    dst = buffer_shim_data_at(self, length);
    bytes = vsnprintf(dst, 1 + required, format, ap);
    va_end(ap);

    return bytes < 0 ? -1 : 0;
}
