/* Auto-generated C accessor shims (field address helpers). */
#include <stddef.h>
#include <stdint.h>
#include "c2r_accessors.h"

// === C2R_INCLUDE_BEGIN ===
// (auto-appended includes will be inserted here)
#include "/data/home/wangshb/c2-rust_framework/ComparisonMethod/Our/projects/buffer/src/buffer.h"
// === C2R_INCLUDE_END ===

// === C2R_FIELD_PTR_DEFS_BEGIN ===
// (auto-appended definitions will be inserted here)
void* c2r_field_ptr_buffer_t__len(void* base) {
    return (void*)(&((buffer_t*)base)->len);
}
void* c2r_field_ptr_buffer_t__alloc(void* base) {
    return (void*)(&((buffer_t*)base)->alloc);
}
void* c2r_field_ptr_buffer_t__data(void* base) {
    return (void*)(&((buffer_t*)base)->data);
}
// === C2R_FIELD_PTR_DEFS_END ===
