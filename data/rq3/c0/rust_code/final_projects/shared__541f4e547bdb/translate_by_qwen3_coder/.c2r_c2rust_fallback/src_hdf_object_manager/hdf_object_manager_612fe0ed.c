# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/framework/core/shared/src/hdf_object_manager.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/out/hispark_taurus/hispark_taurus_mini_system/config.h" 1
# 2 "<built-in>" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/framework/core/shared/src/hdf_object_manager.c" 2








# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/framework/core/shared/include/hdf_object_manager.h" 1
# 12 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/framework/core/shared/include/hdf_object_manager.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_object.h" 1
# 32 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_object.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_base.h" 1
# 33 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_base.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/adapter/khdf/liteos/osal/include/hdf_types.h" 1
# 34 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/adapter/khdf/liteos/osal/include/hdf_types.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdbool.h" 1 3
# 35 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/adapter/khdf/liteos/osal/include/hdf_types.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stddef.h" 1 3
# 19 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stddef.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1 3
# 41 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef unsigned wchar_t;
# 59 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef struct { long long __ll; long double __ld; } max_align_t;
# 75 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef unsigned int size_t;
# 121 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef int ptrdiff_t;
# 20 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stddef.h" 2 3
# 36 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/adapter/khdf/liteos/osal/include/hdf_types.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdint.h" 1 3
# 20 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdint.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1 3
# 116 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef unsigned int uintptr_t;
# 131 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef int intptr_t;
# 147 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef signed char int8_t;




typedef signed short int16_t;




typedef signed int int32_t;




typedef signed long long int64_t;




typedef signed long long intmax_t;




typedef unsigned char uint8_t;




typedef unsigned short uint16_t;




typedef unsigned int uint32_t;




typedef unsigned long long uint64_t;
# 197 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef unsigned long long uintmax_t;
# 21 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdint.h" 2 3

typedef int8_t int_fast8_t;
typedef int64_t int_fast64_t;

typedef int8_t int_least8_t;
typedef int16_t int_least16_t;
typedef int32_t int_least32_t;
typedef int64_t int_least64_t;

typedef uint8_t uint_fast8_t;
typedef uint64_t uint_fast64_t;

typedef uint8_t uint_least8_t;
typedef uint16_t uint_least16_t;
typedef uint32_t uint_least32_t;
typedef uint64_t uint_least64_t;
# 95 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdint.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/stdint.h" 1 3
typedef int32_t int_fast16_t;
typedef int32_t int_fast32_t;
typedef uint32_t uint_fast16_t;
typedef uint32_t uint_fast32_t;
# 96 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdint.h" 2 3
# 37 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/adapter/khdf/liteos/osal/include/hdf_types.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/string.h" 1 3







# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/features.h" 1 3
# 9 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/string.h" 2 3
# 25 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/string.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1 3
# 345 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef struct __locale_struct *locale_t;
# 26 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/string.h" 2 3

void *memcpy (void *restrict, const void *restrict, size_t);
void *memmove (void *, const void *, size_t);
void *memset (void *, int, size_t);
int memcmp (const void *, const void *, size_t);
void *memchr (const void *, int, size_t);

char *strcpy (char *restrict, const char *restrict);
char *strncpy (char *restrict, const char *restrict, size_t);

char *strcat (char *restrict, const char *restrict);
char *strncat (char *restrict, const char *restrict, size_t);

int strcmp (const char *, const char *);
int strncmp (const char *, const char *, size_t);

int strcoll (const char *, const char *);
size_t strxfrm (char *restrict, const char *restrict, size_t);

char *strchr (const char *, int);
char *strrchr (const char *, int);

size_t strcspn (const char *, const char *);
size_t strspn (const char *, const char *);
char *strpbrk (const char *, const char *);
char *strstr (const char *, const char *);
char *strtok (char *restrict, const char *restrict);

size_t strlen (const char *);

char *strerror (int);


# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/strings.h" 1 3
# 11 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/strings.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1 3
# 12 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/strings.h" 2 3




int bcmp (const void *, const void *, size_t);
void bcopy (const void *, void *, size_t);
void bzero (void *, size_t);



int ffs (int);
int ffsl (long);
int ffsll (long long);


int strcasecmp (const char *, const char *);
int strncasecmp (const char *, const char *, size_t);

int strcasecmp_l (const char *, const char *, locale_t);
int strncasecmp_l (const char *, const char *, size_t, locale_t);
# 60 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/string.h" 2 3





char *strtok_r (char *restrict, const char *restrict, char **restrict);
int strerror_r (int, char *, size_t);
char *stpcpy(char *restrict, const char *restrict);
char *stpncpy(char *restrict, const char *restrict, size_t);
size_t strnlen (const char *, size_t);
char *strdup (const char *);
char *strndup (const char *, size_t);
char *strsignal(int);
char *strerror_l (int, locale_t);
int strcoll_l (const char *, const char *, locale_t);
size_t strxfrm_l (char *restrict, const char *restrict, size_t, locale_t);




void *memccpy (void *restrict, const void *restrict, int, size_t);



char *strsep(char **, const char *);
size_t strlcat (char *, const char *, size_t);
size_t strlcpy (char *, const char *, size_t);
void explicit_bzero (void *, size_t);




int strverscmp (const char *, const char *);
char *strchrnul(const char *, int);
char *strcasestr(const char *, const char *);
void *memmem(const void *, size_t, const void *, size_t);
void *memrchr(const void *, int, size_t);
void *mempcpy(void *, const void *, size_t);

char *basename(const char *);
# 38 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/adapter/khdf/liteos/osal/include/hdf_types.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/sys/types.h" 1 3
# 54 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/sys/types.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1 3
# 65 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef long long time_t;




typedef long long suseconds_t;
# 86 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef struct cpu_set_t { unsigned long __bits[32/(8 * sizeof(unsigned long int))]; } cpu_set_t;




struct sched_param {
  int sched_priority;
};




typedef struct __pthread_attr_s {
  unsigned int detachstate;
  unsigned int schedpolicy;
  struct sched_param schedparam;
  unsigned int inheritsched;
  unsigned int scope;
  unsigned int stackaddr_set;
  void* stackaddr;
  unsigned int stacksize_set;
  size_t stacksize;

  cpu_set_t cpuset;

} pthread_attr_t;
# 126 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef int ssize_t;
# 141 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef int register_t;
# 192 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef unsigned long long u_int64_t;
# 203 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef unsigned mode_t;




typedef unsigned int nlink_t;




typedef long long off_t;
typedef long long loff_t;
typedef off_t off64_t;




typedef unsigned long long ino_t;




typedef unsigned long long dev_t;




typedef long blksize_t;




typedef long long blkcnt_t;




typedef unsigned long long fsblkcnt_t;




typedef unsigned long long fsfilcnt_t;
# 262 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef void * timer_t;




typedef int clockid_t;




typedef long clock_t;
# 294 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef int pid_t;




typedef unsigned id_t;




typedef unsigned uid_t;




typedef unsigned gid_t;




typedef int key_t;




typedef unsigned useconds_t;





typedef long pthread_t;




typedef int pthread_once_t;




typedef int pthread_key_t;




typedef int pthread_condattr_t;
# 55 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/sys/types.h" 2 3


typedef unsigned char u_int8_t;
typedef unsigned short u_int16_t;
typedef unsigned u_int32_t;
typedef char *caddr_t;
typedef unsigned char u_char;
typedef unsigned short u_short, ushort;
typedef unsigned u_int, uint;
typedef unsigned long u_long, ulong;
typedef long long quad_t;
typedef unsigned long long u_quad_t;
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/endian.h" 1 3
# 10 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/endian.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1 3
# 11 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/endian.h" 2 3
# 23 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/endian.h" 3
static inline uint16_t __bswap16(uint16_t __x)
{
 return __x<<8 | __x>>8;
}

static inline uint32_t __bswap32(uint32_t __x)
{
 return __x>>24 | __x>>8&0xff00 | __x<<8&0xff0000 | __x<<24;
}

static inline uint64_t __bswap64(uint64_t __x)
{
 return __bswap32(__x)+0ULL<<32 | __bswap32(__x>>32);
}
# 84 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/endian.h" 3
static inline uint32_t
be32dec(const void *pp)
{
    uint8_t const *p = (uint8_t const *)pp;

    return (((unsigned)p[0] << 24) | (p[1] << 16) | (p[2] << 8) | p[3]);
}

static inline void
be32enc(void *pp, uint32_t u)
{
    uint8_t *p = (uint8_t *)pp;

    p[0] = (u >> 24) & 0xff;
    p[1] = (u >> 16) & 0xff;
    p[2] = (u >> 8) & 0xff;
    p[3] = u & 0xff;
}

static inline void
be64enc(void *pp, uint64_t u)
{
    uint8_t *p = (uint8_t *)pp;

    be32enc(p, (uint32_t)(u >> 32));
    be32enc(p + 4, (uint32_t)(u & 0xffffffffU));
}
# 68 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/sys/types.h" 2 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/sys/select.h" 1 3
# 16 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/sys/select.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1 3
# 277 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
struct timeval { time_t tv_sec; suseconds_t tv_usec; };

struct timeval64 { long long tv_sec; long long tv_usec; };





struct timespec { time_t tv_sec; long tv_nsec; };

struct timespec64 { long long tv_sec; long long tv_nsec; };
# 365 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 3
typedef unsigned long long sigset_t;
# 17 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/sys/select.h" 2 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/signal.h" 1 3
# 31 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/signal.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1 3
# 32 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/signal.h" 2 3
# 48 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/signal.h" 3
typedef struct sigaltstack stack_t;



# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/signal.h" 1 3
# 10 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/signal.h" 3
typedef int greg_t, gregset_t[18];
typedef struct sigcontext {
 unsigned long trap_no, error_code, oldmask;
 unsigned long arm_r0, arm_r1, arm_r2, arm_r3;
 unsigned long arm_r4, arm_r5, arm_r6, arm_r7;
 unsigned long arm_r8, arm_r9, arm_r10, arm_fp;
 unsigned long arm_ip, arm_sp, arm_lr, arm_pc;
 unsigned long arm_cpsr, fault_address;
} mcontext_t;






struct sigaltstack {
 void *ss_sp;
 int ss_flags;
 size_t ss_size;
};

typedef struct ucontext {
 unsigned long uc_flags;
 struct ucontext *uc_link;
 stack_t uc_stack;
 mcontext_t uc_mcontext;
 sigset_t uc_sigmask;
 unsigned long long uc_regspace[64];
} ucontext_t;
# 53 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/signal.h" 2 3
# 98 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/signal.h" 3
union sigval {
 int sival_int;
 void *sival_ptr;
};

typedef struct {
 int si_signo, si_errno, si_code;
 union {
  char __pad[128 - 2*sizeof(int) - sizeof(long)];
  struct {
   union {
    struct {
     pid_t si_pid;
     uid_t si_uid;
    } __piduid;
    struct {
     int si_timerid;
     int si_overrun;
    } __timer;
   } __first;
   union {
    union sigval si_value;
    struct {
     int si_status;
     clock_t si_utime, si_stime;
    } __sigchld;
   } __second;
  } __si_common;
 } __si_fields;
} siginfo_t;
# 137 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/signal.h" 3
struct sigaction {
 union {
  void (*sa_handler)(int);
  void (*sa_sigaction)(int, siginfo_t *, void *);
 } sa_sigactionhandler;
 sigset_t sa_mask;
 int sa_flags;
 void (*sa_restorer)(void);
};






struct sigevent {
 union sigval sigev_value;
 int sigev_signo;
 int sigev_notify;
 union {
  char __pad[64 - 2*sizeof(int) - sizeof(union sigval)];
  pid_t sigev_notify_thread_id;
  struct {
   void (*sigev_notify_function)(union sigval);
   pthread_attr_t *sigev_notify_attributes;
  } __sev_thread;
 } __sev_fields;
};
# 178 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/signal.h" 3
int pthread_sigmask(int, const sigset_t *, sigset_t *);
int sigfillset(sigset_t *);
int sigdelset(sigset_t *, int);



void (*sigset(int, void (*)(int)))(int);
# 204 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/signal.h" 3
typedef void (*sig_t)(int);






typedef void (*sighandler_t)(int);
void (*bsd_signal(int, void (*)(int)))(int);
# 222 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/signal.h" 3
typedef int sig_atomic_t;

void (*signal(int, void (*)(int)))(int);
int raise(int);
# 18 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/sys/select.h" 2 3

# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/fs/include/vfs_config.h" 1 3
# 36 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/fs/include/vfs_config.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/common/los_config.h" 1 3
# 39 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/common/los_config.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h" 1 3
# 40 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_err.h" 1 3
# 40 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_err.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_typedef.h" 1 3
# 42 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_typedef.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_builddef.h" 1 3
# 43 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_typedef.h" 2 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_toolchain.h" 1 3
# 44 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_typedef.h" 2 3
# 55 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_typedef.h" 3
typedef unsigned char UINT8;
typedef unsigned short UINT16;
typedef unsigned int UINT32;
typedef signed char INT8;
typedef signed short INT16;
typedef signed int INT32;
typedef float FLOAT;
typedef double DOUBLE;
typedef char CHAR;







typedef unsigned long long UINT64;
typedef signed long long INT64;
typedef unsigned int UINTPTR;
typedef signed int INTPTR;
# 83 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_typedef.h" 3
typedef INT32 ssize_t;
typedef UINT32 size_t;


typedef UINTPTR AARCHPTR;
typedef size_t BOOL;
# 205 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_typedef.h" 3
typedef int status_t;
typedef unsigned long vaddr_t;
typedef unsigned long PADDR_T;
typedef unsigned long VADDR_T;
typedef unsigned long paddr_t;
typedef unsigned long DMA_ADDR_T;
typedef unsigned long ADDR_T;
typedef unsigned long VM_OFFSET_T;
typedef unsigned long PTE_T;
typedef unsigned int ULONG_T;
typedef int STATUS_T;
# 41 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_err.h" 2 3
# 70 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_err.h" 3
typedef void (*LOS_ERRORHANDLE_FUNC)(CHAR *fileName,
                                     UINT32 lineNo,
                                     UINT32 errorNo,
                                     UINT32 paraLen,
                                     void *para);
# 98 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_err.h" 3
extern UINT32 LOS_ErrHandle(CHAR *fileName, UINT32 lineNo,
                            UINT32 errorNo, UINT32 paraLen,
                            void *para);







extern void LOS_SetErrHandleHook(LOS_ERRORHANDLE_FUNC fun);

enum LOS_MOUDLE_ID {
    LOS_MOD_SYS = 0x0,
    LOS_MOD_MEM = 0x1,
    LOS_MOD_TSK = 0x2,
    LOS_MOD_SWTMR = 0x3,
    LOS_MOD_TICK = 0x4,
    LOS_MOD_MSG = 0x5,
    LOS_MOD_QUE = 0x6,
    LOS_MOD_SEM = 0x7,
    LOS_MOD_MBOX = 0x8,
    LOS_MOD_HWI = 0x9,
    LOS_MOD_HWWDG = 0xa,
    LOS_MOD_CACHE = 0xb,
    LOS_MOD_HWTMR = 0xc,
    LOS_MOD_MMU = 0xd,

    LOS_MOD_LOG = 0xe,
    LOS_MOD_ERR = 0xf,

    LOS_MOD_EXC = 0x10,
    LOS_MOD_CSTK = 0x11,

    LOS_MOD_MPU = 0x12,
    LOS_MOD_NMHWI = 0x13,
    LOS_MOD_TRACE = 0x14,
    LOS_MOD_KNLSTAT = 0x15,
    LOS_MOD_EVTTIME = 0x16,
    LOS_MOD_THRDCPUP = 0x17,
    LOS_MOD_IPC = 0x18,
    LOS_MOD_STKMON = 0x19,
    LOS_MOD_TIMER = 0x1a,
    LOS_MOD_RESLEAKMON = 0x1b,
    LOS_MOD_EVENT = 0x1c,
    LOS_MOD_MUX = 0X1d,
    LOS_MOD_CPUP = 0x1e,
    LOS_MOD_HOOK = 0x1f,
    LOS_MOD_PERF = 0x20,
    LOS_MOD_PM = 0x21,
    LOS_MOD_SHELL = 0x31,
    LOS_MOD_DRIVER = 0x41,
    LOS_MOD_BUTT
};
# 41 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h" 2 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_errno.h" 1 3
# 42 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h" 2 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/arch/arm/arm/include/hal_timer.h" 1 3
# 41 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/arch/arm/arm/include/hal_timer.h" 3
extern UINT32 HalClockFreqRead(void);
extern void HalClockFreqWrite(UINT32 freq);
extern void HalClockStart(void);
extern void HalClockIrqClear(void);
extern void HalClockInit(void);
extern UINT64 HalClockGetCycles(void);
extern void HalDelayUs(UINT32 usecs);
extern UINT32 HalClockGetTickTimerCycles(void);
extern UINT64 HalClockTickTimerReload(UINT64 cycles);

extern UINT32 HrtimersInit(void);
extern void HrtimerClockIrqClear(void);
extern void HrtimerClockStart(UINT32 period);
extern void HrtimerClockStop(void);
extern UINT32 HrtimerClockValueGet(void);
extern void HrtimerClockInit(void);
# 43 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h" 2 3
# 86 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h" 3
extern UINT32 g_sysClock;





extern UINT32 g_tickPerSecond;
# 115 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h" 3
extern void LOS_GetCpuCycle(UINT32 *puwCntHi, UINT32 *puwCntLo);
# 134 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h" 3
extern UINT64 LOS_CurrNanosec(void);
# 153 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h" 3
extern void LOS_Udelay(UINT32 usecs);
# 172 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h" 3
extern void LOS_Mdelay(UINT32 usecs);
# 40 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/common/los_config.h" 2 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/base/include/los_vm_zone.h" 1 3
# 35 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/base/include/los_vm_zone.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/device/board/hisilicon/hispark_taurus/liteos_a/board/target_config.h" 1 3
# 36 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/base/include/los_vm_zone.h" 2 3
# 41 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/common/los_config.h" 2 3
# 52 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/common/los_config.h" 3
extern CHAR __int_stack_start;
extern CHAR __rodata_start;
extern CHAR __rodata_end;
extern CHAR __bss_start;
extern CHAR __bss_end;
extern CHAR __text_start;
extern CHAR __text_end;
extern CHAR __ram_data_start;
extern CHAR __ram_data_end;
extern UINT32 __heap_start;
extern UINT32 __heap_end;
# 474 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/common/los_config.h" 3
extern UINT32 OsMain(void);

typedef void (*SystemRebootFunc)(void);
void OsSetRebootHook(SystemRebootFunc func);
SystemRebootFunc OsGetRebootHook(void);
# 37 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/fs/include/vfs_config.h" 2 3
# 104 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/fs/include/vfs_config.h" 3
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/net/lwip-2.1/porting/include/lwip/lwipopts.h" 1 3
# 105 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/fs/include/vfs_config.h" 2 3
# 20 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/sys/select.h" 2 3







typedef unsigned long fd_mask;

typedef struct {
 unsigned long fds_bits[((512 + 128) + (0 + 256)) / 8 / sizeof(long)];
} fd_set;






int select (int, fd_set *restrict, fd_set *restrict, fd_set *restrict, struct timeval *restrict);
int pselect (int, fd_set *restrict, fd_set *restrict, fd_set *restrict, const struct timespec *restrict, const sigset_t *restrict);
# 69 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/sys/types.h" 2 3
# 39 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/adapter/khdf/liteos/osal/include/hdf_types.h" 2
# 34 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_base.h" 2








typedef enum {
    HDF_SUCCESS = 0,
    HDF_FAILURE = -1,
    HDF_ERR_NOT_SUPPORT = -2,
    HDF_ERR_INVALID_PARAM = -3,
    HDF_ERR_INVALID_OBJECT = -4,
    HDF_ERR_MALLOC_FAIL = -6,
    HDF_ERR_TIMEOUT = -7,
    HDF_ERR_THREAD_CREATE_FAIL = -10,
    HDF_ERR_QUEUE_FULL = -15,
    HDF_ERR_DEVICE_BUSY = -16,
    HDF_ERR_IO = -17,
    HDF_ERR_BAD_FD = -18,
    HDF_ERR_NOPERM = -19,
    HDF_ERR_OUT_OF_RANGE = -20,



    HDF_BSP_ERR_OP = ((-100) + (-1)),
    HDF_ERR_BSP_PLT_API_ERR = ((-100) + (-2)),
    HDF_PAL_ERR_DEV_CREATE = ((-100) + (-3)),
    HDF_PAL_ERR_INNER = ((-100) + (-4)),



    HDF_DEV_ERR_NO_MEMORY = ((-200) + (-1)),
    HDF_DEV_ERR_NO_DEVICE = ((-200) + (-2)),
    HDF_DEV_ERR_NO_DEVICE_SERVICE = ((-200) + (-3)),
    HDF_DEV_ERR_DEV_INIT_FAIL = ((-200) + (-4)),
    HDF_DEV_ERR_PUBLISH_FAIL = ((-200) + (-5)),
    HDF_DEV_ERR_ATTACHDEV_FAIL = ((-200) + (-6)),
    HDF_DEV_ERR_NODATA = ((-200) + (-7)),
    HDF_DEV_ERR_NORANGE = ((-200) + (-8)),
    HDF_DEV_ERR_OP = ((-200) + (-10)),
    HDF_DEV_ERR_NETDOWN = ((-200) + (-11)),
} HDF_STATUS;
# 33 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_object.h" 2






struct HdfObject {
    int32_t objectId;
};
# 13 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/framework/core/shared/include/hdf_object_manager.h" 2





enum {
    HDF_OBJECT_ID_DEVMGR_SERVICE = 0,
    HDF_OBJECT_ID_DEVSVC_MANAGER,
    HDF_OBJECT_ID_DEVHOST_SERVICE,
    HDF_OBJECT_ID_DRIVER_INSTALLER,
    HDF_OBJECT_ID_DRIVER_LOADER,
    HDF_OBJECT_ID_DEVICE,
    HDF_OBJECT_ID_DEVICE_TOKEN,
    HDF_OBJECT_ID_DEVICE_SERVICE,
    HDF_OBJECT_ID_REMOTE_SERVICE,
    HDF_OBJECT_ID_MAX
};

struct HdfObjectCreator {
    struct HdfObject *(*Create)(void);
    void (*Release)(struct HdfObject *);
};

const struct HdfObjectCreator *HdfObjectManagerGetCreators(int objectId);
struct HdfObject *HdfObjectManagerGetObject(int objectId);
void HdfObjectManagerFreeObject(struct HdfObject *object);
# 10 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/framework/core/shared/src/hdf_object_manager.c" 2

struct HdfObject *HdfObjectManagerGetObject(int objectId)
{
    struct HdfObject *object = ((void*)0);
    const struct HdfObjectCreator *targetCreator = HdfObjectManagerGetCreators(objectId);
    if ((targetCreator != ((void*)0)) && (targetCreator->Create != ((void*)0))) {
        object = targetCreator->Create();
        if (object != ((void*)0)) {
            object->objectId = objectId;
        }
    }
    return object;
}

void HdfObjectManagerFreeObject(struct HdfObject *object)
{
    const struct HdfObjectCreator *targetCreator = ((void*)0);
    if (object == ((void*)0)) {
        return;
    }
    targetCreator = HdfObjectManagerGetCreators(object->objectId);
    if ((targetCreator == ((void*)0)) || (targetCreator->Release == ((void*)0))) {
        return;
    }
    targetCreator->Release(object);
}
