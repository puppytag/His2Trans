#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "bzlib.h"

static void roundtrip(const unsigned char *src, unsigned int src_len) {
  const unsigned int extra = 601u;
  unsigned int comp_cap = src_len + (src_len / 100u) + extra;
  char *comp = (char *)malloc(comp_cap);
  assert(comp);

  unsigned int comp_len = comp_cap;
  int rc = BZ2_bzBuffToBuffCompress(
      comp, &comp_len, (char *)src, src_len, /*blockSize100k=*/9, /*verbosity=*/0, /*workFactor=*/30);
  assert(rc == BZ_OK);
  assert(comp_len > 0);

  unsigned int dec_cap = src_len + 16u;
  char *dec = (char *)malloc(dec_cap);
  assert(dec);

  unsigned int dec_len = dec_cap;
  rc = BZ2_bzBuffToBuffDecompress(dec, &dec_len, comp, comp_len, /*small=*/0, /*verbosity=*/0);
  assert(rc == BZ_OK);
  assert(dec_len == src_len);
  assert(memcmp(dec, src, src_len) == 0);

  free(dec);
  free(comp);
}

int main(void) {
  const char *ver = BZ2_bzlibVersion();
  assert(ver && ver[0] != '\0');

  roundtrip((const unsigned char *)"hello bzip2 test", (unsigned int)strlen("hello bzip2 test"));
  roundtrip((const unsigned char *)"", 0u);
  roundtrip((const unsigned char *)"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", (unsigned int)strlen("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));

  printf("bzip2 tests: ok (bzlibVersion=%s)\n", ver);
  return 0;
}

