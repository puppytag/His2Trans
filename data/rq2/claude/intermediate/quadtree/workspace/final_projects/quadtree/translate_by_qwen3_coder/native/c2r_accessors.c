/* Auto-generated C accessor shims (field address helpers). */
#include <stddef.h>
#include <stdint.h>
#include "c2r_accessors.h"

// === C2R_INCLUDE_BEGIN ===
// (auto-appended includes will be inserted here)
#include "quadtree.h"

// === C2R_INCLUDE_END ===

// === C2R_FIELD_PTR_DEFS_BEGIN ===
// (auto-appended definitions will be inserted here)
void* c2r_field_ptr_quadtree_point_t__x(void* base) {
    return (void*)(&((quadtree_point_t*)base)->x);
}
void* c2r_field_ptr_quadtree_point_t__y(void* base) {
    return (void*)(&((quadtree_point_t*)base)->y);
}
void* c2r_field_ptr_quadtree_node_t__point(void* base) {
    return (void*)(&((quadtree_node_t*)base)->point);
}
void* c2r_field_ptr_quadtree_bounds_t__se(void* base) {
    return (void*)(&((quadtree_bounds_t*)base)->se);
}
void* c2r_field_ptr_quadtree_bounds_t__width(void* base) {
    return (void*)(&((quadtree_bounds_t*)base)->width);
}
void* c2r_field_ptr_quadtree_bounds_t__height(void* base) {
    return (void*)(&((quadtree_bounds_t*)base)->height);
}
void* c2r_field_ptr_quadtree_bounds_t__nw(void* base) {
    return (void*)(&((quadtree_bounds_t*)base)->nw);
}
void* c2r_field_ptr_quadtree_node_t__bounds(void* base) {
    return (void*)(&((quadtree_node_t*)base)->bounds);
}
void* c2r_field_ptr_quadtree_node_t__nw(void* base) {
    return (void*)(&((quadtree_node_t*)base)->nw);
}
void* c2r_field_ptr_quadtree_node_t__sw(void* base) {
    return (void*)(&((quadtree_node_t*)base)->sw);
}
void* c2r_field_ptr_quadtree_node_t__se(void* base) {
    return (void*)(&((quadtree_node_t*)base)->se);
}
void* c2r_field_ptr_quadtree_node_t__ne(void* base) {
    return (void*)(&((quadtree_node_t*)base)->ne);
}
void* c2r_field_ptr_quadtree_node_t__key(void* base) {
    return (void*)(&((quadtree_node_t*)base)->key);
}
void* c2r_field_ptr_quadtree_t__key_free(void* base) {
    return (void*)(&((quadtree_t*)base)->key_free);
}
void* c2r_field_ptr_quadtree_t__root(void* base) {
    return (void*)(&((quadtree_t*)base)->root);
}
void* c2r_field_ptr_quadtree_t__length(void* base) {
    return (void*)(&((quadtree_t*)base)->length);
}
// === C2R_FIELD_PTR_DEFS_END ===
