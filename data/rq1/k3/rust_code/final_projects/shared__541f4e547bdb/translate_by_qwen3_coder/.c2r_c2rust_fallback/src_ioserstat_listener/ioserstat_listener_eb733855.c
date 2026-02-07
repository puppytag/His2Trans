# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/framework/core/shared/src/ioserstat_listener.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/framework/core/shared/src/ioserstat_listener.c" 2








# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/ioservstat_listener.h" 1
# 12 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/ioservstat_listener.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_dlist.h" 1
# 35 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_dlist.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_base.h" 1
# 33 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_base.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/osal/uhdf/hdf_types.h" 1
# 19 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/osal/uhdf/hdf_types.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdbool.h" 1
# 20 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/osal/uhdf/hdf_types.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stddef.h" 1
# 19 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stddef.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1
# 41 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef unsigned wchar_t;
# 59 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef struct { long long __ll; long double __ld; } max_align_t;
# 75 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef unsigned int size_t;
# 121 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef int ptrdiff_t;
# 20 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stddef.h" 2
# 21 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/osal/uhdf/hdf_types.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdint.h" 1
# 20 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdint.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1
# 116 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef unsigned int uintptr_t;
# 131 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef int intptr_t;
# 147 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef signed char int8_t;




typedef signed short int16_t;




typedef signed int int32_t;




typedef signed long long int64_t;




typedef signed long long intmax_t;




typedef unsigned char uint8_t;




typedef unsigned short uint16_t;




typedef unsigned int uint32_t;




typedef unsigned long long uint64_t;
# 197 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef unsigned long long uintmax_t;
# 21 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdint.h" 2

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
# 95 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdint.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/stdint.h" 1
typedef int32_t int_fast16_t;
typedef int32_t int_fast32_t;
typedef uint32_t uint_fast16_t;
typedef uint32_t uint_fast32_t;
# 96 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdint.h" 2
# 22 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/osal/uhdf/hdf_types.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/string.h" 1







# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/features.h" 1
# 9 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/string.h" 2
# 25 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/string.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1
# 345 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef struct __locale_struct *locale_t;
# 26 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/string.h" 2

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


# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/strings.h" 1
# 11 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/strings.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1
# 12 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/strings.h" 2




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
# 60 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/string.h" 2





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
# 23 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/osal/uhdf/hdf_types.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/limits.h" 1





# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1
# 7 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/limits.h" 2

# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/common/los_config.h" 1
# 39 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/common/los_config.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h" 1
# 40 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_err.h" 1
# 40 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_err.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_typedef.h" 1
# 42 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_typedef.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_builddef.h" 1
# 43 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_typedef.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_toolchain.h" 1
# 44 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_typedef.h" 2
# 55 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_typedef.h"
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
# 83 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_typedef.h"
typedef INT32 ssize_t;
typedef UINT32 size_t;


typedef UINTPTR AARCHPTR;
typedef size_t BOOL;
# 205 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_typedef.h"
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
# 41 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_err.h" 2
# 70 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_err.h"
typedef void (*LOS_ERRORHANDLE_FUNC)(CHAR *fileName,
                                     UINT32 lineNo,
                                     UINT32 errorNo,
                                     UINT32 paraLen,
                                     void *para);
# 98 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_err.h"
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
# 41 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_errno.h" 1
# 42 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/arch/arm/arm/include/hal_timer.h" 1
# 41 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/arch/arm/arm/include/hal_timer.h"
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
# 43 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h" 2
# 86 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h"
extern UINT32 g_sysClock;





extern UINT32 g_tickPerSecond;
# 115 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h"
extern void LOS_GetCpuCycle(UINT32 *puwCntHi, UINT32 *puwCntLo);
# 134 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h"
extern UINT64 LOS_CurrNanosec(void);
# 153 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h"
extern void LOS_Udelay(UINT32 usecs);
# 172 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/include/los_tick.h"
extern void LOS_Mdelay(UINT32 usecs);
# 40 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/common/los_config.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/base/include/los_vm_zone.h" 1
# 35 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/base/include/los_vm_zone.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/device/board/hisilicon/hispark_taurus/liteos_a/board/target_config.h" 1
# 36 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/base/include/los_vm_zone.h" 2
# 41 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/common/los_config.h" 2
# 52 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/common/los_config.h"
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
# 474 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/kernel/liteos_a/kernel/common/los_config.h"
extern UINT32 OsMain(void);

typedef void (*SystemRebootFunc)(void);
void OsSetRebootHook(SystemRebootFunc func);
SystemRebootFunc OsGetRebootHook(void);
# 9 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/limits.h" 2
# 73 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/limits.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/limits.h" 1
# 74 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/limits.h" 2
# 24 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/osal/uhdf/hdf_types.h" 2
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
# 36 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_dlist.h" 2








struct DListHead {
    struct DListHead *next;
    struct DListHead *prev;
};
# 56 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_dlist.h"
static inline void DListHeadInit(struct DListHead *head)
{
    head->next = head;
    head->prev = head;
}
# 69 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_dlist.h"
static inline _Bool DListIsEmpty(const struct DListHead *head)
{
    return (head->next == head) ? 1 : 0;
}
# 81 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_dlist.h"
static inline void DListRemove(struct DListHead *entry)
{
    entry->prev->next = entry->next;
    entry->next->prev = entry->prev;

    entry->prev = ((void*)0);
    entry->next = ((void*)0);
}
# 99 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_dlist.h"
static inline void DListInsertHead(struct DListHead *entry, struct DListHead *head)
{
    entry->next = head->next;
    entry->prev = head;

    head->next->prev = entry;
    head->next = entry;
}
# 117 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_dlist.h"
static inline void DListInsertTail(struct DListHead *entry, struct DListHead *head)
{
    entry->next = head;
    entry->prev = head->prev;

    head->prev->next = entry;
    head->prev = entry;
}
# 135 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_dlist.h"
static inline void DListMerge(struct DListHead *list, struct DListHead *head)
{
    list->next->prev = head;
    list->prev->next = head->next;

    head->next->prev = list->prev;
    head->next = list->next;

    DListHeadInit(list);
}
# 153 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_dlist.h"
static inline int DListGetCount(const struct DListHead *head)
{
    struct DListHead *next = head->next;
    int count = 0;
    while (next != head) {
        next = next->next;
        count++;
    }
    return count;
}
# 13 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/ioservstat_listener.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h" 1
# 33 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_device_class.h" 1
# 35 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_device_class.h"
typedef enum {
    DEVICE_CLASS_DEFAULT = 0x1 << 0,
    DEVICE_CLASS_PLAT = 0x1 << 1,
    DEVICE_CLASS_SENSOR = 0x1 << 2,
    DEVICE_CLASS_INPUT = 0x1 << 3,
    DEVICE_CLASS_DISPLAY = 0x1 << 4,
    DEVICE_CLASS_AUDIO = 0x1 << 5,
    DEVICE_CLASS_CAMERA = 0x1 << 6,
    DEVICE_CLASS_USB = 0x1 << 7,
    DEVICE_CLASS_USERAUTH = 0x1 << 8,
    DEVICE_CLASS_HIMEDIACOMM = 0x1 << 9,
    DEVICE_CLASS_MAX = 0x1 << 10,
} DeviceClass;
# 34 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h" 2

# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_object.h" 1
# 39 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_object.h"
struct HdfObject {
    int32_t objectId;
};
# 36 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h" 1
# 38 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
typedef uint16_t char16_t;


struct HdfSBuf;
struct HdfSBufImpl;
struct HdfRemoteService;






enum HdfSbufType {
    SBUF_RAW = 0,
    SBUF_IPC,
    SBUF_IPC_HW,
    SBUF_TYPE_MAX,
};
# 67 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteBuffer(struct HdfSBuf *sbuf, const void *data, uint32_t writeSize);
# 79 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteUnpadBuffer(struct HdfSBuf *sbuf, const uint8_t *data, uint32_t writeSize);
# 90 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteUint64(struct HdfSBuf *sbuf, uint64_t value);
# 101 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteUint32(struct HdfSBuf *sbuf, uint32_t value);
# 112 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteUint16(struct HdfSBuf *sbuf, uint16_t value);
# 123 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteUint8(struct HdfSBuf *sbuf, uint8_t value);
# 134 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteInt64(struct HdfSBuf *sbuf, int64_t value);
# 145 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteInt32(struct HdfSBuf *sbuf, int32_t value);
# 156 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteInt16(struct HdfSBuf *sbuf, int16_t value);
# 167 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteInt8(struct HdfSBuf *sbuf, int8_t value);
# 178 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteString(struct HdfSBuf *sbuf, const char *value);
# 191 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteString16(struct HdfSBuf *sbuf, const char16_t *value, uint32_t size);
# 203 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteFloat(struct HdfSBuf *sbuf, float data);
# 215 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteDouble(struct HdfSBuf *sbuf, double data);
# 227 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufWriteFileDescriptor(struct HdfSBuf *sbuf, int fd);
# 238 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
int32_t HdfSbufWriteRemoteService(struct HdfSBuf *sbuf, const struct HdfRemoteService *service);
# 250 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
struct HdfRemoteService *HdfSbufReadRemoteService(struct HdfSBuf *sbuf);
# 261 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
int HdfSbufReadFileDescriptor(struct HdfSBuf *sbuf);
# 274 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufReadDouble(struct HdfSBuf *sbuf, double *data);
# 286 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufReadFloat(struct HdfSBuf *sbuf, float *data);
# 298 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
const char16_t *HdfSbufReadString16(struct HdfSBuf *sbuf);
# 312 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufReadBuffer(struct HdfSBuf *sbuf, const void **data, uint32_t *readSize);
# 323 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
const uint8_t *HdfSbufReadUnpadBuffer(struct HdfSBuf *sbuf, size_t length);
# 334 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufReadUint64(struct HdfSBuf *sbuf, uint64_t *value);
# 345 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufReadUint32(struct HdfSBuf *sbuf, uint32_t *value);
# 356 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufReadUint16(struct HdfSBuf *sbuf, uint16_t *value);
# 367 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufReadUint8(struct HdfSBuf *sbuf, uint8_t *value);
# 378 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufReadInt64(struct HdfSBuf *sbuf, int64_t *value);
# 389 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufReadInt32(struct HdfSBuf *sbuf, int32_t *value);
# 400 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufReadInt16(struct HdfSBuf *sbuf, int16_t *value);
# 411 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
_Bool HdfSbufReadInt8(struct HdfSBuf *sbuf, int8_t *value);
# 422 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
const char *HdfSbufReadString(struct HdfSBuf *sbuf);
# 433 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
uint8_t *HdfSbufGetData(const struct HdfSBuf *sbuf);
# 442 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
void HdfSbufFlush(struct HdfSBuf *sbuf);
# 452 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
size_t HdfSbufGetCapacity(const struct HdfSBuf *sbuf);
# 462 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
size_t HdfSbufGetDataSize(const struct HdfSBuf *sbuf);
# 472 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
void HdfSbufSetDataSize(struct HdfSBuf *sbuf, size_t size);
# 482 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
struct HdfSBuf *HdfSbufObtain(size_t capacity);
# 491 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
struct HdfSBuf *HdfSbufObtainDefaultSize(void);
# 504 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
struct HdfSBuf *HdfSbufBind(uintptr_t base, size_t size);
# 513 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
void HdfSbufRecycle(struct HdfSBuf *sbuf);
# 524 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
struct HdfSBuf *HdfSbufMove(struct HdfSBuf *sbuf);
# 535 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
struct HdfSBuf *HdfSbufCopy(const struct HdfSBuf *sbuf);
# 545 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
void HdfSbufTransDataOwnership(struct HdfSBuf *sbuf);
# 555 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
struct HdfSBuf *HdfSbufTypedObtain(uint32_t type);
# 566 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
struct HdfSBuf *HdfSbufTypedObtainInplace(uint32_t type, struct HdfSBufImpl *impl);
# 577 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
struct HdfSBuf *HdfSbufTypedObtainCapacity(uint32_t type, size_t capacity);
# 591 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
struct HdfSBuf *HdfSbufTypedBind(uint32_t type, uintptr_t base, size_t size);
# 601 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/utils/hdf_sbuf.h"
struct HdfSBufImpl *HdfSbufGetImpl(struct HdfSBuf *sbuf);
# 37 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h" 2





struct HdfDevEventlistener;
struct HdfIoService;
# 57 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
typedef int (*OnEventReceived)(void *priv, uint32_t id, struct HdfSBuf *data);
# 72 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
typedef int (*OnDevEventReceive)(
    struct HdfDevEventlistener *listener, struct HdfIoService *service, uint32_t id, struct HdfSBuf *data);






struct HdfDevEventlistener {

    OnEventReceived callBack;

    OnDevEventReceive onReceive;

    struct DListHead listNode;

    void *priv;
};






struct HdfIoDispatcher {



    int (*Dispatch)(struct HdfObject *service, int cmdId, struct HdfSBuf *data, struct HdfSBuf *reply);
};






struct HdfIoService {

    struct HdfObject object;

    struct HdfObject *target;

    struct HdfIoDispatcher *dispatcher;

    void *priv;
};






struct HdfIoServiceGroup {

    struct HdfObject object;
};
# 137 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
struct HdfIoServiceGroup *HdfIoServiceGroupObtain(void);
# 146 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
void HdfIoServiceGroupRecycle(struct HdfIoServiceGroup *group);
# 158 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
int32_t HdfIoServiceGroupAddService(struct HdfIoServiceGroup *group, struct HdfIoService *service);
# 170 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
void HdfIoServiceGroupRemoveService(struct HdfIoServiceGroup *group, struct HdfIoService *service);
# 183 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
int32_t HdfIoServiceGroupRegisterListener(struct HdfIoServiceGroup *group, struct HdfDevEventlistener *listener);
# 197 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
int32_t HdfIoServiceGroupRegisterListenerWithSchedPolicy(
    struct HdfIoServiceGroup *group, struct HdfDevEventlistener *listener, int policy);
# 210 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
int32_t HdfIoServiceGroupUnregisterListener(struct HdfIoServiceGroup *group, struct HdfDevEventlistener *listener);
# 223 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
struct HdfIoService *HdfIoServiceBind(const char *serviceName);
# 232 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
void HdfIoServiceRecycle(struct HdfIoService *service);
# 245 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
int HdfDeviceRegisterEventListener(struct HdfIoService *target, struct HdfDevEventlistener *listener);
# 259 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
int32_t HdfDeviceRegisterEventListenerWithSchedPolicy(
    struct HdfIoService *target, struct HdfDevEventlistener *listener, int policy);
# 272 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
int HdfDeviceUnregisterEventListener(struct HdfIoService *target, struct HdfDevEventlistener *listener);
# 282 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
int HdfIoserviceGetListenerCount(const struct HdfIoService *service);
# 292 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
int HdfIoserviceGroupGetListenerCount(const struct HdfIoServiceGroup *group);
# 302 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
int HdfIoserviceGroupGetServiceCount(const struct HdfIoServiceGroup *group);
# 315 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_io_service_if.h"
int32_t HdfGetServiceNameByDeviceClass(DeviceClass deviceClass, struct HdfSBuf *reply);


int32_t HdfIoServiceDispatch(struct HdfIoService *ioService, int cmdId, struct HdfSBuf *data, struct HdfSBuf *reply);
# 14 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/ioservstat_listener.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_service_status.h" 1
# 38 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/hdf_service_status.h"
struct ServiceStatusListener;




enum ServiceStatusType {

    SERVIE_STATUS_START,

    SERVIE_STATUS_CHANGE,

    SERVIE_STATUS_STOP,

    SERVIE_STATUS_REGISTER,

    SERVIE_STATUS_MAX,
};





struct ServiceStatus {

    const char* serviceName;

    uint16_t deviceClass;

    uint16_t status;

    const char *info;
};







typedef void (*OnServiceStatusReceived)(struct ServiceStatusListener *listener, struct ServiceStatus *status);




struct ServiceStatusListener {

    OnServiceStatusReceived callback;





    void *priv;
};
# 15 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/core/ioservstat_listener.h" 2





struct IoServiceStatusListener {
    struct ServiceStatusListener svcstatListener;
    struct HdfDevEventlistener ioservListener;
    struct DListHead node;
    uint16_t deviceClass;
};

struct ServiceStatusListener *IoServiceStatusListenerNewInstance(void);
void IoServiceStatusListenerFree(struct ServiceStatusListener *listener);
# 10 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/framework/core/shared/src/ioserstat_listener.c" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/framework/core/shared/include/hdf_service_status_inner.h" 1
# 17 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/framework/core/shared/include/hdf_service_status_inner.h"
struct HdfSBuf;

enum ServiceStatusListenerCmd {
    SERVIE_STATUS_LISTENER_NOTIFY,
    SERVIE_STATUS_LISTENER_MAX,
};

int ServiceStatusMarshalling(struct ServiceStatus *status, struct HdfSBuf *buf);
int ServiceStatusUnMarshalling(struct ServiceStatus *status, struct HdfSBuf *buf);
# 11 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/framework/core/shared/src/ioserstat_listener.c" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/osal/shared/osal_mem.h" 1
# 49 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/osal/shared/osal_mem.h"
void *OsalMemAlloc(size_t size);
# 60 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/osal/shared/osal_mem.h"
void *OsalMemCalloc(size_t size);
# 72 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/osal/shared/osal_mem.h"
void *OsalMemAllocAlign(size_t alignment, size_t size);
# 81 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/interfaces/inner_api/osal/shared/osal_mem.h"
void OsalMemFree(void *mem);
# 12 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/drivers/hdf_core/framework/core/shared/src/ioserstat_listener.c" 2

static int OnIoServiceEventReceive(
    struct HdfDevEventlistener *listener, struct HdfIoService *service, uint32_t id, struct HdfSBuf *data)
{
    if (listener == ((void*)0) || service == ((void*)0) || data == ((void*)0)) {
        return HDF_ERR_INVALID_PARAM;
    }
    (void)id;
    struct ServiceStatus status = { 0 };
    if (ServiceStatusUnMarshalling(&status, data) != HDF_SUCCESS) {
        return HDF_ERR_INVALID_PARAM;
    }

    struct IoServiceStatusListener *statusListener = listener->priv;
    if (statusListener->svcstatListener.callback != ((void*)0) &&
        (statusListener->deviceClass & status.deviceClass)) {
        statusListener->svcstatListener.callback(&statusListener->svcstatListener, &status);
    }

    return HDF_SUCCESS;
}

struct ServiceStatusListener *IoServiceStatusListenerNewInstance(void)
{
    struct IoServiceStatusListener *listener = OsalMemCalloc(sizeof(struct IoServiceStatusListener));
    if (listener == ((void*)0)) {
        return ((void*)0);
    }

    listener->ioservListener.onReceive = OnIoServiceEventReceive;
    listener->ioservListener.priv = (void *)listener;

    return &listener->svcstatListener;
}

void IoServiceStatusListenerFree(struct ServiceStatusListener *listener)
{
    if (listener == ((void*)0)) {
        return;
    }
    struct IoServiceStatusListener *ioservListener
        = (struct IoServiceStatusListener *)((char *)(listener) - (char *)&((struct IoServiceStatusListener *)0)->svcstatListener);
    OsalMemFree(ioservListener);
}
