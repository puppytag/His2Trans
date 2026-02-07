#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "quadtree.h"

static uint32_t lcg32(uint32_t *state) {
  // Numerical Recipes LCG (deterministic, cheap).
  *state = (*state * 1664525u) + 1013904223u;
  return *state;
}

static int max_depth(quadtree_node_t *n) {
  if (!n) return 0;
  int d_nw = max_depth(n->nw);
  int d_ne = max_depth(n->ne);
  int d_sw = max_depth(n->sw);
  int d_se = max_depth(n->se);
  int max = d_nw;
  if (d_ne > max) max = d_ne;
  if (d_sw > max) max = d_sw;
  if (d_se > max) max = d_se;
  return 1 + max;
}

typedef struct walk_stats {
  unsigned int nodes;
  unsigned int leaves_with_points;
  unsigned int empty_leaves;
  unsigned int internal_nodes;
} walk_stats_t;

static walk_stats_t g_stats;

static void on_descent(quadtree_node_t *node) {
  g_stats.nodes++;
  if (quadtree_node_isleaf(node)) {
    g_stats.leaves_with_points++;
  } else if (quadtree_node_isempty(node)) {
    g_stats.empty_leaves++;
  } else {
    g_stats.internal_nodes++;
  }
}

static void on_ascent(quadtree_node_t *node) {
  (void)node;
}

int main(void) {
  // Use bounds that yield fractional quadrant boundaries so integer points
  // won't land on split lines (node_contains_ uses strict inequalities).
  const double minx = -1.0, miny = -1.0, maxx = 1000.0, maxy = 1000.0;
  quadtree_t *qt = quadtree_new(minx, miny, maxx, maxy);
  assert(qt);

  // Enable key freeing to also exercise duplicate-point reset paths.
  qt->key_free = free;

  enum { N = 5000, Q = 20000 };
  int xs[N];
  int ys[N];

  // Generate unique integer points within (0..999).
  // We ensure uniqueness by using a 1000x1000 occupancy bitmap.
  unsigned char *seen = (unsigned char *)calloc(1000u * 1000u, 1);
  assert(seen);

  uint32_t rng = 1u;
  int inserted = 0;
  while (inserted < N) {
    int x = (int)(lcg32(&rng) % 1000u);
    int y = (int)(lcg32(&rng) % 1000u);
    unsigned int idx = (unsigned int)(y * 1000 + x);
    if (seen[idx]) continue;
    seen[idx] = 1;

    int *key = (int *)malloc(sizeof(int));
    assert(key);
    *key = inserted;
    int ok = quadtree_insert(qt, (double)x, (double)y, key);
    assert(ok == 1);

    xs[inserted] = x;
    ys[inserted] = y;
    inserted++;
  }

  assert(qt->length == (unsigned int)N);

  // Verify search works for inserted points.
  for (int i = 0; i < N; i++) {
    quadtree_point_t *p = quadtree_search(qt, (double)xs[i], (double)ys[i]);
    assert(p);
    assert(p->x == (double)xs[i]);
    assert(p->y == (double)ys[i]);
  }

  // Duplicate insertion: does not increase length and returns 0 in this impl.
  {
    int *dup_key = (int *)malloc(sizeof(int));
    assert(dup_key);
    *dup_key = 123456;
    int rc = quadtree_insert(qt, (double)xs[0], (double)ys[0], dup_key);
    assert(rc == 0);
    assert(qt->length == (unsigned int)N);
    quadtree_point_t *p = quadtree_search(qt, (double)xs[0], (double)ys[0]);
    assert(p);
  }

  // Walk stats: count nodes and point-bearing leaves.
  g_stats = (walk_stats_t){0};
  quadtree_walk(qt->root, on_descent, on_ascent);
  assert(g_stats.leaves_with_points == (unsigned int)N);
  assert(g_stats.nodes > g_stats.leaves_with_points);

  int depth = max_depth(qt->root);
  assert(depth > 0);

  // Stress: random searches (inspired by public/quadtree's hot loop),
  // but without external dependencies.
  // NOTE: This quadtree implementation's quadtree_search() is only safe for
  // points that exist in the tree; searching a missing point can recurse into
  // an "empty" node and crash. So the hot-loop only queries inserted points.
  clock_t t0 = clock();
  unsigned int hits = 0;
  for (int i = 0; i < Q; i++) {
    int idx = (int)(lcg32(&rng) % (uint32_t)N);
    int x = xs[idx];
    int y = ys[idx];
    quadtree_point_t *p = quadtree_search(qt, (double)x, (double)y);
    assert(p);
    hits++;
  }
  clock_t t1 = clock();

  double sec = (double)(t1 - t0) / (double)CLOCKS_PER_SEC;
  printf("quadtree tests: ok\n");
  printf("  inserted=%d length=%u\n", inserted, qt->length);
  printf("  walk.nodes=%u leaves_with_points=%u empty_leaves=%u internal_nodes=%u\n",
         g_stats.nodes, g_stats.leaves_with_points, g_stats.empty_leaves, g_stats.internal_nodes);
  printf("  max_depth=%d\n", depth);
  printf("  random_searches=%d hits=%u time_sec=%.6f\n", Q, hits, sec);

  free(seen);
  quadtree_free(qt);
  return 0;
}
