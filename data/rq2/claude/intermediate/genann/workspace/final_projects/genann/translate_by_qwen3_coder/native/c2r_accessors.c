/* Auto-generated C accessor shims (field address helpers). */
#include <stddef.h>
#include <stdint.h>
#include "c2r_accessors.h"

// === C2R_INCLUDE_BEGIN ===
// (auto-appended includes will be inserted here)
#include "genann.h"

// === C2R_INCLUDE_END ===

// === C2R_FIELD_PTR_DEFS_BEGIN ===
// (auto-appended definitions will be inserted here)
void* c2r_field_ptr_genann__weight(void* base) {
    return (void*)(&((genann*)base)->weight);
}
void* c2r_field_ptr_genann__total_weights(void* base) {
    return (void*)(&((genann*)base)->total_weights);
}
// === C2R_FIELD_PTR_DEFS_END ===
