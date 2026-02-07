# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/src/app_provision.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/src/app_provision.c" 2
# 16 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/src/app_provision.c"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/include/app_provision.h" 1
# 19 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/include/app_provision.h"
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
# 20 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/include/app_provision.h" 2

# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/include/app_verify_pub.h" 1
# 19 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/include/app_verify_pub.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdbool.h" 1
# 20 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/include/app_verify_pub.h" 2








typedef enum {
    V_OK = 0,


    V_ERR_GET_CERT_INFO = 0xef000002,
    V_ERR_UNTRUSTED_CERT = 0xef000003,
    V_ERR_INTEGRITY = 0xef000004,
    V_ERR_GET_SIGNHEAD = 0xef000005,
    V_ERR_GET_SIGN_BLOCK = 0xef000006,
    V_ERR_GET_HASH_DIFF = 0xef000007,
    V_ERR_INVALID_CONTENT_TAG = 0xef000008,
    V_ERR_INVALID_HASH_ALG = 0xef000009,
    V_ERR_GET_ROOT_HASH = 0xef00000a,
    V_ERR_CALC_BLOCK_HASH = 0xef00000c,
    V_ERR_PARSE_PKC7_DATA = 0xef00000d,
    V_ERR_VERIFY_CERT_CHAIN = 0xef00000e,
    V_ERR_VERIFY_SIGNATURE = 0xef00000f,
    V_ERR_GET_CERT_TYPE = 0xef000010,


    V_ERR_GET_PROFILE_DATA = 0xef000011,
    V_ERR_GET_PARSE_PROFILE = 0xef000012,
    V_ERR_PROF_CONTENT_INVALID = 0xef000013,
    V_ERR_VERFIY_PROF_CERT = 0xef000014,
    V_ERR_GET_CERT_PK = 0xef000015,
    V_ERR_GET_APPID = 0xef000016,
    V_ERR_INVALID_DISP_TYPE = 0xef000017,
    V_ERR_INVALID_APP_BUNDLE = 0xef000018,
    V_ERR_INVALID_DATE = 0xef000019,
    V_ERR_INVALID_DEVID = 0xef00001a,


    V_ERR_FILE_OPEN = 0xef00001b,
    V_ERR_FILE_STAT = 0xef00001c,
    V_ERR_FILE_LENGTH = 0xef00001d,


    V_ERR_MEMSET = 0xef00001e,
    V_ERR_MEMCPY = 0xef00001f,
    V_ERR_MALLOC = 0xef000020,


    V_ERR = 0xffffffff,
} AppVErrCode;

typedef struct {
    int32_t notBefore;
    int32_t notAfter;
} ProfValidity;

typedef struct {
    char *developerId;
    unsigned char *devCert;
    unsigned char *releaseCert;
    char *bundleName;
    char *appFeature;
} ProfBundleInfo;

typedef struct {
    int32_t restricNum;
    char **restricPermission;
    int32_t permissionNum;
    char **permission;
} ProfPermission;

typedef struct {
    char *devIdType;
    int32_t devidNum;
    char **deviceId;
} ProfDebugInfo;

typedef struct {
    int32_t versionCode;
    char *versionName;
    char *uuid;
    char *type;
    char *appDistType;
    ProfValidity validity;
    ProfBundleInfo bundleInfo;
    ProfPermission permission;
    ProfDebugInfo debugInfo;
    char *issuer;
    char *appid;
} ProfileProf;

typedef struct {
    char *pk;
    int32_t len;
} AppSignPk;

struct VfyRst;
typedef int32_t (*GetSignPk)(struct VfyRst *verifyRst, AppSignPk *pk);
typedef void (*FreeSignPK)(AppSignPk *pk);
typedef int32_t (*MessageFunc)(unsigned char operationResult, const char *bundleName, unsigned char errCode);

typedef struct VfyRst {
    ProfileProf profile;
} VerifyResult;


int32_t APPVERI_AppVerify(const char *filePath, VerifyResult *verifyRst);


void APPVERI_FreeVerifyRst(VerifyResult *verifyRst);


int32_t APPVERI_SetDebugMode(_Bool mode);


int32_t APPVERI_GetUnsignedFileLength(const char *filePath);


void APPVERI_RegisterMsgFunc(MessageFunc messageFunc);
void APPVERI_SetActsMode(_Bool mode);
int32_t APPVERI_IsActsMode(void);
# 22 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/include/app_provision.h" 2
# 38 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/include/app_provision.h"
int32_t ParseProfile(const char *buf, int32_t len, ProfileProf *pf);
void ProfFreeData(ProfileProf *pf);
int32_t VerifyProfileContent(const ProfileProf *pf);
# 17 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/src/app_provision.c" 2

# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/string.h" 1







# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/features.h" 1
# 9 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/string.h" 2
# 25 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/string.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1
# 75 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef unsigned int size_t;
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
# 19 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/src/app_provision.c" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/include/app_common.h" 1
# 21 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/include/app_common.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/products/ipcamera/app_verify_base.h" 1
# 18 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/products/ipcamera/app_verify_base.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/include/app_verify_hal.h" 1
# 29 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/include/app_verify_hal.h"
typedef int32_t (*GetDeviceUdid)(unsigned char *udid, int32_t size);

typedef struct {
    GetDeviceUdid devUdidFunc;
} ProductDiff;

void RegistHalFunc();
int32_t InquiryDeviceUdid(unsigned char *udid, int32_t size);
# 19 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/products/ipcamera/app_verify_base.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/log.h" 1
# 19 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/log.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/hilog_cp.h" 1
# 19 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/hilog_cp.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/hiview_log.h" 1
# 63 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/hiview_log.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdarg.h" 1
# 10 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdarg.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1
# 29 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef __builtin_va_list va_list;
# 11 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdarg.h" 2
# 64 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/hiview_log.h" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdio.h" 1
# 26 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdio.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1
# 34 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef __builtin_va_list __isoc_va_list;
# 126 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef int ssize_t;
# 213 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef long long off_t;
typedef long long loff_t;
typedef off_t off64_t;
# 355 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef struct _IO_FILE FILE;
# 27 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdio.h" 2
# 56 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdio.h"
typedef union _G_fpos64_t {
 char __opaque[16];
 long long __lldata;
 double __align;
} fpos_t;

extern FILE *const stdin;
extern FILE *const stdout;
extern FILE *const stderr;





FILE *fopen(const char *restrict, const char *restrict);
FILE *freopen(const char *restrict, const char *restrict, FILE *restrict);
int fclose(FILE *);

int remove(const char *);
int rename(const char *, const char *);

int feof(FILE *);
int ferror(FILE *);
int fflush(FILE *);
void clearerr(FILE *);

int fseek(FILE *, long, int);
long ftell(FILE *);
void rewind(FILE *);

int fgetpos(FILE *restrict, fpos_t *restrict);
int fsetpos(FILE *, const fpos_t *);

size_t fread(void *restrict, size_t, size_t, FILE *restrict);
size_t fwrite(const void *restrict, size_t, size_t, FILE *restrict);

int fgetc(FILE *);
int getc(FILE *);
int getchar(void);
int ungetc(int, FILE *);

int fputc(int, FILE *);
int putc(int, FILE *);
int putchar(int);

char *fgets(char *restrict, int, FILE *restrict);




int fputs(const char *restrict, FILE *restrict);
int puts(const char *);

int printf(const char *restrict, ...);
int fprintf(FILE *restrict, const char *restrict, ...);
int sprintf(char *restrict, const char *restrict, ...);
int snprintf(char *restrict, size_t, const char *restrict, ...);

int vprintf(const char *restrict, __isoc_va_list);
int vfprintf(FILE *restrict, const char *restrict, __isoc_va_list);
int vsprintf(char *restrict, const char *restrict, __isoc_va_list);
int vsnprintf(char *restrict, size_t, const char *restrict, __isoc_va_list);

int scanf(const char *restrict, ...);
int fscanf(FILE *restrict, const char *restrict, ...);
int sscanf(const char *restrict, const char *restrict, ...);
int vscanf(const char *restrict, __isoc_va_list);
int vfscanf(FILE *restrict, const char *restrict, __isoc_va_list);
int vsscanf(const char *restrict, const char *restrict, __isoc_va_list);

void perror(const char *);

int setvbuf(FILE *restrict, char *restrict, int, size_t);
void setbuf(FILE *restrict, char *restrict);

char *tmpnam(char *);
FILE *tmpfile(void);




FILE *fmemopen(void *restrict, size_t, const char *restrict);
FILE *open_memstream(char **, size_t *);
FILE *fdopen(int, const char *);
FILE *popen(const char *, const char *);
int pclose(FILE *);
int fileno(FILE *);
int fseeko(FILE *, off_t, int);
off_t ftello(FILE *);
void dprintf(const char *restrict, ...);
int vdprintf(int, const char *restrict, __isoc_va_list);
void flockfile(FILE *);
int ftrylockfile(FILE *);
void funlockfile(FILE *);
int getc_unlocked(FILE *);
int getchar_unlocked(void);
int putc_unlocked(int, FILE *);
int putchar_unlocked(int);
ssize_t getdelim(char **restrict, size_t *restrict, int, FILE *restrict);
ssize_t getline(char **restrict, size_t *restrict, FILE *restrict);
int renameat(int, const char *, int, const char *);
char *ctermid(char *);







char *tempnam(const char *, const char *);




char *cuserid(char *);
void setlinebuf(FILE *);
void setbuffer(FILE *, char *, size_t);
int fgetc_unlocked(FILE *);
int fputc_unlocked(int, FILE *);
int fflush_unlocked(FILE *);
size_t fread_unlocked(void *, size_t, size_t, FILE *);
size_t fwrite_unlocked(const void *, size_t, size_t, FILE *);
void clearerr_unlocked(FILE *);
int feof_unlocked(FILE *);
int ferror_unlocked(FILE *);
int fileno_unlocked(FILE *);
int getw(FILE *);
int putw(int, FILE *);
char *fgetln(FILE *, size_t *);
int asprintf(char **, const char *, ...);
int vasprintf(char **, const char *, __isoc_va_list);



char *fgets_unlocked(char *, int, FILE *);
int fputs_unlocked(const char *, FILE *);

typedef ssize_t (cookie_read_function_t)(void *, char *, size_t);
typedef ssize_t (cookie_write_function_t)(void *, const char *, size_t);
typedef int (cookie_seek_function_t)(void *, off_t *, int);
typedef int (cookie_close_function_t)(void *);

typedef struct _IO_cookie_io_functions_t {
 cookie_read_function_t *read;
 cookie_write_function_t *write;
 cookie_seek_function_t *seek;
 cookie_close_function_t *close;
} cookie_io_functions_t;

FILE *fopencookie(void *, const char *, cookie_io_functions_t);
# 65 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/hiview_log.h" 2
# 78 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/hiview_log.h"
typedef enum {

    HILOG_MODULE_HIVIEW = 0,

    HILOG_MODULE_SAMGR,

    HILOG_MODULE_UPDATE,

    HILOG_MODULE_ACE,

    HILOG_MODULE_APP,

    HILOG_MODULE_MAX
} HiLogModuleType;
# 128 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/hiview_log.h"
typedef enum {
    LOG_TYPE_MIN = 0,

    LOG_INIT = 1,

    LOG_CORE = 3,
    LOG_TYPE_MAX
} LogType;
# 154 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/hiview_log.h"
typedef enum {

    LOG_DEBUG = 3,

    LOG_INFO = 4,

    LOG_WARN = 5,

    LOG_ERROR = 6,

    LOG_FATAL = 7,
} LogLevel;
# 191 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/hiview_log.h"
int HiLogPrint(LogType type, LogLevel level, unsigned int domain, const char* tag, const char* fmt, ...)
    __attribute__((format(os_log, 5, 6)));
# 210 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/hiview_log.h"
int HiLogPrintArgs(LogType bufID, LogLevel prio,
    unsigned int domain, const char* tag, const char* fmt, va_list ap);
# 313 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/hiview_log.h"
int FlushHilog(void);



struct HiLogEntry {
    unsigned int len;
    unsigned int hdrSize;
    unsigned int pid : 16;
    unsigned int taskId : 16;
    unsigned int sec;
    unsigned int nsec;
    unsigned int reserved;
    char msg[0];
};
# 20 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/hilog_cp.h" 2
# 20 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/hiviewdfx/hilog_lite/interfaces/native/innerkits/hilog/log.h" 2
# 20 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/products/ipcamera/app_verify_base.h" 2
# 38 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/products/ipcamera/app_verify_base.h"
void RegistProductFunc(ProductDiff *productFunc);
# 22 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/include/app_common.h" 2
# 106 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/include/app_common.h"
long long HapGetInt64(const unsigned char *buf, int32_t len);
int32_t HapGetInt(const unsigned char *buf, int32_t len);
uint32_t HapGetUnsignedInt(const unsigned char *buf, int32_t len);
short HapGetShort(const unsigned char *buf, int32_t len);
void HapPutInt32(unsigned char *buf, int32_t len, int32_t value);
# 20 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/src/app_provision.c" 2

# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/cJSON/cJSON.h" 1
# 86 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/cJSON/cJSON.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stddef.h" 1
# 19 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stddef.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1
# 41 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef unsigned wchar_t;
# 59 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef struct { long long __ll; long double __ld; } max_align_t;
# 121 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h"
typedef int ptrdiff_t;
# 20 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stddef.h" 2
# 87 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/cJSON/cJSON.h" 2
# 111 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/cJSON/cJSON.h"
typedef struct cJSON
{

    struct cJSON *next;
    struct cJSON *prev;

    struct cJSON *child;


    int type;


    char *valuestring;





    int valueint;


    double valuedouble;


    char *string;
} cJSON;

typedef struct cJSON_Hooks
{

      void *( *malloc_fn)(size_t sz);
      void ( *free_fn)(void *ptr);
} cJSON_Hooks;

typedef int cJSON_bool;
# 154 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/cJSON/cJSON.h"
const char* cJSON_Version(void);


void cJSON_InitHooks(cJSON_Hooks* hooks);



cJSON * cJSON_Parse(const char *value);
cJSON * cJSON_ParseWithLength(const char *value, size_t buffer_length);


cJSON * cJSON_ParseWithOpts(const char *value, const char **return_parse_end, cJSON_bool require_null_terminated);
cJSON * cJSON_ParseWithLengthOpts(const char *value, size_t buffer_length, const char **return_parse_end, cJSON_bool require_null_terminated);


char * cJSON_Print(const cJSON *item);

char * cJSON_PrintUnformatted(const cJSON *item);

char * cJSON_PrintBuffered(const cJSON *item, int prebuffer, cJSON_bool fmt);


cJSON_bool cJSON_PrintPreallocated(cJSON *item, char *buffer, const int length, const cJSON_bool format);

void cJSON_Delete(cJSON *item);


int cJSON_GetArraySize(const cJSON *array);

cJSON * cJSON_GetArrayItem(const cJSON *array, int index);

cJSON * cJSON_GetObjectItem(const cJSON * const object, const char * const string);
cJSON * cJSON_GetObjectItemCaseSensitive(const cJSON * const object, const char * const string);
cJSON_bool cJSON_HasObjectItem(const cJSON *object, const char *string);

const char * cJSON_GetErrorPtr(void);


char * cJSON_GetStringValue(const cJSON * const item);



double cJSON_GetNumberValue(const cJSON * const item);


cJSON_bool cJSON_IsInvalid(const cJSON * const item);
cJSON_bool cJSON_IsFalse(const cJSON * const item);
cJSON_bool cJSON_IsTrue(const cJSON * const item);
cJSON_bool cJSON_IsBool(const cJSON * const item);
cJSON_bool cJSON_IsNull(const cJSON * const item);



cJSON_bool cJSON_IsNumber(const cJSON * const item);
cJSON_bool cJSON_IsString(const cJSON * const item);
cJSON_bool cJSON_IsArray(const cJSON * const item);
cJSON_bool cJSON_IsObject(const cJSON * const item);
cJSON_bool cJSON_IsRaw(const cJSON * const item);


cJSON * cJSON_CreateNull(void);
cJSON * cJSON_CreateTrue(void);
cJSON * cJSON_CreateFalse(void);
cJSON * cJSON_CreateBool(cJSON_bool boolean);
cJSON * cJSON_CreateNumber(double num);



cJSON * cJSON_CreateString(const char *string);

cJSON * cJSON_CreateRaw(const char *raw);
cJSON * cJSON_CreateArray(void);
cJSON * cJSON_CreateObject(void);



cJSON * cJSON_CreateStringReference(const char *string);


cJSON * cJSON_CreateObjectReference(const cJSON *child);
cJSON * cJSON_CreateArrayReference(const cJSON *child);



cJSON * cJSON_CreateIntArray(const int *numbers, int count);
cJSON * cJSON_CreateFloatArray(const float *numbers, int count);
cJSON * cJSON_CreateDoubleArray(const double *numbers, int count);
cJSON * cJSON_CreateStringArray(const char *const *strings, int count);


cJSON_bool cJSON_AddItemToArray(cJSON *array, cJSON *item);
cJSON_bool cJSON_AddItemToObject(cJSON *object, const char *string, cJSON *item);



cJSON_bool cJSON_AddItemToObjectCS(cJSON *object, const char *string, cJSON *item);

cJSON_bool cJSON_AddItemReferenceToArray(cJSON *array, cJSON *item);
cJSON_bool cJSON_AddItemReferenceToObject(cJSON *object, const char *string, cJSON *item);


cJSON * cJSON_DetachItemViaPointer(cJSON *parent, cJSON * const item);
cJSON * cJSON_DetachItemFromArray(cJSON *array, int which);
void cJSON_DeleteItemFromArray(cJSON *array, int which);
cJSON * cJSON_DetachItemFromObject(cJSON *object, const char *string);
cJSON * cJSON_DetachItemFromObjectCaseSensitive(cJSON *object, const char *string);
void cJSON_DeleteItemFromObject(cJSON *object, const char *string);
void cJSON_DeleteItemFromObjectCaseSensitive(cJSON *object, const char *string);


cJSON_bool cJSON_InsertItemInArray(cJSON *array, int which, cJSON *newitem);
cJSON_bool cJSON_ReplaceItemViaPointer(cJSON * const parent, cJSON * const item, cJSON * replacement);
cJSON_bool cJSON_ReplaceItemInArray(cJSON *array, int which, cJSON *newitem);
cJSON_bool cJSON_ReplaceItemInObject(cJSON *object,const char *string,cJSON *newitem);
cJSON_bool cJSON_ReplaceItemInObjectCaseSensitive(cJSON *object,const char *string,cJSON *newitem);


cJSON * cJSON_Duplicate(const cJSON *item, cJSON_bool recurse);





cJSON_bool cJSON_Compare(const cJSON * const a, const cJSON * const b, const cJSON_bool case_sensitive);




void cJSON_Minify(char *json);



cJSON* cJSON_AddNullToObject(cJSON * const object, const char * const name);
cJSON* cJSON_AddTrueToObject(cJSON * const object, const char * const name);
cJSON* cJSON_AddFalseToObject(cJSON * const object, const char * const name);
cJSON* cJSON_AddBoolToObject(cJSON * const object, const char * const name, const cJSON_bool boolean);



cJSON* cJSON_AddNumberToObject(cJSON * const object, const char * const name, const double number);
cJSON* cJSON_AddStringToObject(cJSON * const object, const char * const name, const char * const string);
cJSON* cJSON_AddRawToObject(cJSON * const object, const char * const name, const char * const raw);
cJSON* cJSON_AddObjectToObject(cJSON * const object, const char * const name);
cJSON* cJSON_AddArrayToObject(cJSON * const object, const char * const name);







double cJSON_SetNumberHelper(cJSON *object, double number);


char* cJSON_SetValuestring(cJSON *object, const char *valuestring);
# 321 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/cJSON/cJSON.h"
void * cJSON_malloc(size_t size);
void cJSON_free(void *object);
# 22 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/src/app_provision.c" 2
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h" 1
# 21 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securectype.h" 1
# 318 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securectype.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdlib.h" 1
# 21 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdlib.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1
# 22 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdlib.h" 2

int atoi (const char *);
long atol (const char *);
long long atoll (const char *);
double atof (const char *);

float strtof (const char *restrict, char **restrict);
double strtod (const char *restrict, char **restrict);
long double strtold (const char *restrict, char **restrict);

long strtol (const char *restrict, char **restrict, int);
unsigned long strtoul (const char *restrict, char **restrict, int);
long long strtoll (const char *restrict, char **restrict, int);
unsigned long long strtoull (const char *restrict, char **restrict, int);

int rand (void);
void srand (unsigned);

void *malloc (size_t);
void *calloc (size_t, size_t);
void *realloc (void *, size_t);
void free (void *);
void *aligned_alloc(size_t, size_t);
void *zalloc(size_t);

_Noreturn void abort (void);
int atexit (void (*) (void));
_Noreturn void exit (int);
_Noreturn void _Exit (int);
int at_quick_exit (void (*) (void));
_Noreturn void quick_exit (int);

char *getenv (const char *);

int system (const char *);

void *bsearch (const void *, const void *, size_t, size_t, int (*)(const void *, const void *));
void qsort (void *, size_t, size_t, int (*)(const void *, const void *));

int abs (int);
long labs (long);
long long llabs (long long);

typedef struct { int quot, rem; } div_t;
typedef struct { long quot, rem; } ldiv_t;
typedef struct { long long quot, rem; } lldiv_t;

div_t div (int, int);
ldiv_t ldiv (long, long);
lldiv_t lldiv (long long, long long);

int mblen (const char *, size_t);
int mbtowc (wchar_t *restrict, const char *restrict, size_t);
int wctomb (char *, wchar_t);
size_t mbstowcs (wchar_t *restrict, const char *restrict, size_t);
size_t wcstombs (char *restrict, const wchar_t *restrict, size_t);




size_t __ctype_get_mb_cur_max(void);
# 102 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdlib.h"
int posix_memalign (void **, size_t, size_t);
int setenv (const char *, const char *, int);
int unsetenv (const char *);
int mkstemp (char *);
int mkostemp (char *, int);
char *mkdtemp (char *);
int getsubopt (char **, char *const *, char **);
int rand_r (unsigned *);






char *realpath (const char *restrict, char *restrict);
long int random (void);
void srandom (unsigned int);
char *initstate (unsigned int, char *, size_t);
char *setstate (char *);
int putenv (char *);
int posix_openpt (int);
int grantpt (int);
int unlockpt (int);
char *ptsname (int);
char *l64a (long);
long a64l (const char *);
void setkey (const char *);
double drand48 (void);
double erand48 (unsigned short [3]);
long int lrand48 (void);
long int nrand48 (unsigned short [3]);
long mrand48 (void);
long jrand48 (unsigned short [3]);
void srand48 (long);
unsigned short *seed48 (unsigned short [3]);
void lcong48 (unsigned short [7]);



# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/alloca.h" 1








# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/alltypes.h" 1
# 10 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/alloca.h" 2

void *alloca(size_t);
# 142 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/stdlib.h" 2
char *mktemp (char *);
int mkstemps (char *, int);
int mkostemps (char *, int, int);
void *valloc (size_t);
void *memalign(size_t, size_t);
int getloadavg(double *, int);
int clearenv(void);


void *reallocarray (void *, size_t, size_t);
void qsort_r (void *, size_t, size_t, int (*)(const void *, const void *, void *), void *);



int ptsname_r(int, char *, size_t);
char *ecvt(double, int, int *, int *);
char *fcvt(double, int, int *, int *);
char *gcvt(double, int, char *);
char *secure_getenv(const char *);
struct __locale_struct;
# 319 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securectype.h" 2
# 22 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h" 2
# 39 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/errno.h" 1
# 10 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/errno.h"
# 1 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/bits/errno.h" 1
# 11 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/musl/porting/liteos_a/kernel/include/errno.h" 2


__attribute__((const))

int *__errno_location(void);






extern char *program_invocation_short_name, *program_invocation_name;
# 40 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h" 2
# 50 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
typedef int errno_t;
# 122 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern const char *GetHwSecureCVersion(unsigned short *verNumber);
# 134 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern errno_t memset_s(void *dest, size_t destMax, int c, size_t count);
# 153 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern errno_t memmove_s(void *dest, size_t destMax, const void *src, size_t count);
# 166 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern errno_t memcpy_s(void *dest, size_t destMax, const void *src, size_t count);
# 178 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern errno_t strcpy_s(char *strDest, size_t destMax, const char *strSrc);
# 191 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern errno_t strncpy_s(char *strDest, size_t destMax, const char *strSrc, size_t count);
# 203 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern errno_t strcat_s(char *strDest, size_t destMax, const char *strSrc);
# 217 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern errno_t strncat_s(char *strDest, size_t destMax, const char *strSrc, size_t count);
# 231 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int vsprintf_s(char *strDest, size_t destMax, const char *format,
                           va_list argList) ;
# 245 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int sprintf_s(char *strDest, size_t destMax, const char *format, ...) ;
# 260 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int vsnprintf_s(char *strDest, size_t destMax, size_t count, const char *format,
                            va_list argList) ;
# 275 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int snprintf_s(char *strDest, size_t destMax, size_t count, const char *format,
                           ...) ;
# 290 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int vsnprintf_truncated_s(char *strDest, size_t destMax, const char *format,
                                      va_list argList) ;
# 302 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int snprintf_truncated_s(char *strDest, size_t destMax,
                                     const char *format, ...) ;
# 313 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int scanf_s(const char *format, ...);
# 323 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int vscanf_s(const char *format, va_list argList);
# 334 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int sscanf_s(const char *buffer, const char *format, ...);
# 346 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int vsscanf_s(const char *buffer, const char *format, va_list argList);
# 357 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int fscanf_s(FILE *stream, const char *format, ...);
# 369 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int vfscanf_s(FILE *stream, const char *format, va_list argList);
# 385 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern char *strtok_s(char *strToken, const char *strDelimit, char **context);
# 396 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern char *gets_s(char *buffer, size_t destMax);
# 410 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern errno_t wmemcpy_s(wchar_t *dest, size_t destMax, const wchar_t *src, size_t count);
# 423 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern errno_t wmemmove_s(wchar_t *dest, size_t destMax, const wchar_t *src, size_t count);
# 435 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern errno_t wcscpy_s(wchar_t *strDest, size_t destMax, const wchar_t *strSrc);
# 448 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern errno_t wcsncpy_s(wchar_t *strDest, size_t destMax, const wchar_t *strSrc, size_t count);
# 460 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern errno_t wcscat_s(wchar_t *strDest, size_t destMax, const wchar_t *strSrc);
# 474 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern errno_t wcsncat_s(wchar_t *strDest, size_t destMax, const wchar_t *strSrc, size_t count);
# 486 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern wchar_t *wcstok_s(wchar_t *strToken, const wchar_t *strDelimit, wchar_t **context);
# 499 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int vswprintf_s(wchar_t *strDest, size_t destMax, const wchar_t *format, va_list argList);
# 511 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int swprintf_s(wchar_t *strDest, size_t destMax, const wchar_t *format, ...);
# 521 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int fwscanf_s(FILE *stream, const wchar_t *format, ...);
# 532 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int vfwscanf_s(FILE *stream, const wchar_t *format, va_list argList);
# 541 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int wscanf_s(const wchar_t *format, ...);
# 551 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int vwscanf_s(const wchar_t *format, va_list argList);
# 561 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int swscanf_s(const wchar_t *buffer, const wchar_t *format, ...);
# 572 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/third_party/bounds_checking_function/include/securec.h"
extern int vswscanf_s(const wchar_t *buffer, const wchar_t *format, va_list argList);





extern errno_t strncpy_error(char *strDest, size_t destMax, const char *strSrc, size_t count);
extern errno_t strcpy_error(char *strDest, size_t destMax, const char *strSrc);



extern errno_t memset_sOptAsm(void *dest, size_t destMax, int c, size_t count);
extern errno_t memset_sOptTc(void *dest, size_t destMax, int c, size_t count);
extern errno_t memcpy_sOptAsm(void *dest, size_t destMax, const void *src, size_t count);
extern errno_t memcpy_sOptTc(void *dest, size_t destMax, const void *src, size_t count);
# 23 "/data/home/wangshb/c2-rust_framework/SelfContained/ohos_full/OpenHarmony-v5.0.1-Release/OpenHarmony/base/security/appverify/interfaces/innerkits/appverify_lite/src/app_provision.c" 2

const char APP_GALLERY[] = "app_gallery";
const char ENTERPRISE[] = "enterprise";
const char ENTERPRISE_NORMAL[] = "enterprise_normal";
const char ENTERPRISE_MDM[] = "enterprise_mdm";
const char OS_INTEGRATION[] = "os_integration";
const char INTERNALTESTING[] = "internaltesting";

static void ProfInit(ProfileProf *pf)
{
    errno_t ret = memset_s(pf, sizeof(ProfileProf), 0, sizeof(ProfileProf));
    if (ret != 0) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""memset failed", __FUNCTION__, 35));
        return;
    }
    return;
}

static char *GetStringTag(const cJSON *root, const char *tag)
{
    cJSON *jsonObj = cJSON_GetObjectItem(root, tag);
    if ((jsonObj == ((void*)0)) || (jsonObj->valuestring == ((void*)0))) {
        ((void)HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify", "[%s:%d]: ""failed to get %s", __FUNCTION__, 45, tag));
        return ((void*)0);
    }
    int32_t objLen = strlen(jsonObj->valuestring);
    if (objLen < 0) {
        ((void)HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify", "[%s:%d]: ""len error", __FUNCTION__, 50));
        return ((void*)0);
    }
    char *value = malloc(objLen + 1);
    if (value == ((void*)0)) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""malloc error: %d", __FUNCTION__, 55, objLen + 1));
        return ((void*)0);
    }
    errno_t ret = strcpy_s(value, objLen + 1, jsonObj->valuestring);
    if (ret != 0) {
        do { if ((value) != ((void*)0)) { free(value); (value) = ((void*)0); } } while (0);
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""strcpy error: %d", __FUNCTION__, 61, ret));
        return ((void*)0);
    }
    return value;
}

static void FreeStringAttay(char **array, int32_t num)
{
    if (array == ((void*)0)) {
        return;
    }
    for (int32_t i = 0; i < num; i++) {
        if (array[i] != ((void*)0)) {
            do { if ((array[i]) != ((void*)0)) { free(array[i]); (array[i]) = ((void*)0); } } while (0);
        }
    }
    do { if ((array) != ((void*)0)) { free(array); (array) = ((void*)0); } } while (0);
    return;
}

static char **GetStringArrayTag(const cJSON *root, const char *tag, int32_t *numReturn)
{
    cJSON *jsonObj = cJSON_GetObjectItem(root, tag);
    if (jsonObj == ((void*)0)) {
        ((void)HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify", "[%s:%d]: ""failed to get %s", __FUNCTION__, 85, tag));
        return ((void*)0);
    }
    int32_t num = cJSON_GetArraySize(jsonObj);
    if (num == 0) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""array num 0", __FUNCTION__, 90));
        *numReturn = 0;
        return ((void*)0);
    }
    char **value = malloc(sizeof(char *) * num);
    if (value == ((void*)0)) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""value is null", __FUNCTION__, 96));
        *numReturn = 0;
        return ((void*)0);
    }
    (void)memset_s(value, sizeof(char *) * num, 0, sizeof(char *) * num);

    for (int32_t i = 0; i < num; i++) {
        cJSON *item = cJSON_GetArrayItem(jsonObj, i);
        do { if ((item) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""item"" is null", __FUNCTION__, 104)); goto EXIT; } } while (0);
        if (item->valuestring == ((void*)0)) {
            ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""valuestring is NULL", __FUNCTION__, 106));
            FreeStringAttay(value, num);
            return ((void*)0);
        }
        int32_t len = strlen(item->valuestring);
        value[i] = malloc(len + 1);
        do { if ((value[i]) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""value[i]"" is null", __FUNCTION__, 112)); goto EXIT; } } while (0);

        errno_t ret = strcpy_s(value[i], len + 1, item->valuestring);
        if (ret != 0) {
            ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""str cpy error : %d", __FUNCTION__, 116, ret));
            FreeStringAttay(value, num);
            return ((void*)0);
        }
    }
    *numReturn = num;
    return value;
EXIT:
    FreeStringAttay(value, num);
    return ((void*)0);
}

static int32_t GetProfValidity(const cJSON *root, ProfValidity *profVal)
{
    cJSON *jsonObj = cJSON_GetObjectItem(root, "validity");
    if (jsonObj == ((void*)0)) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""failed to get validity", __FUNCTION__, 132));
        return V_ERR;
    }

    cJSON *notBefore = cJSON_GetObjectItem(jsonObj, "not-before");
    if (notBefore == ((void*)0)) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""failed to get not-before", __FUNCTION__, 138));
        return V_ERR;
    }
    profVal->notBefore = notBefore->valueint;

    cJSON *notAfter = cJSON_GetObjectItem(jsonObj, "not-after");
    if (notAfter == ((void*)0)) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""failed to get not-after", __FUNCTION__, 145));
        return V_ERR;
    }
    profVal->notAfter = notAfter->valueint;
    return V_OK;
}

static int32_t GetProfBundleInfo(const cJSON *root, ProfBundleInfo *profVal)
{
    cJSON *jsonObj = cJSON_GetObjectItem(root, "bundle-info");
    if (jsonObj == ((void*)0)) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""failed to get bundle-info", __FUNCTION__, 156));
        return V_ERR;
    }

    profVal->developerId = GetStringTag(jsonObj, "developer-id");
    do { if ((profVal->developerId) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""profVal->developerId"" is null", __FUNCTION__, 161)); return V_ERR; } } while (0);

    profVal->devCert = (unsigned char *)GetStringTag(jsonObj, "development-certificate");
    if (profVal->devCert == ((void*)0)) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""get development-certificat failed", __FUNCTION__, 165));
        profVal->devCert = malloc(sizeof(char));
        do { if ((profVal->devCert) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""profVal->devCert"" is null", __FUNCTION__, 167)); return V_ERR; } } while (0);
        profVal->devCert[0] = '\0';
    }

    profVal->releaseCert = (unsigned char *)GetStringTag(jsonObj, "distribution-certificate");
    if (profVal->releaseCert == ((void*)0)) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""get distribution-certificat failed", __FUNCTION__, 173));
        profVal->releaseCert = malloc(sizeof(char));
        do { if ((profVal->releaseCert) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""profVal->releaseCert"" is null", __FUNCTION__, 175)); return V_ERR; } } while (0);
        profVal->releaseCert[0] = '\0';
    }

    profVal->bundleName = GetStringTag(jsonObj, "bundle-name");
    do { if ((profVal->bundleName) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""profVal->bundleName"" is null", __FUNCTION__, 180)); return V_ERR; } } while (0);

    profVal->appFeature = GetStringTag(jsonObj, "app-feature");
    do { if ((profVal->appFeature) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""profVal->appFeature"" is null", __FUNCTION__, 183)); return V_ERR; } } while (0);

    return V_OK;
}

static int32_t GetProfPermission(const cJSON *root, ProfPermission *profVal)
{
    cJSON *jsonObj = cJSON_GetObjectItem(root, "permissions");
    if (jsonObj == ((void*)0)) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""failed to get permissions", __FUNCTION__, 192));
        return V_ERR;
    }
    profVal->permission = GetStringArrayTag(jsonObj, "feature-permissions", &profVal->permissionNum);
    profVal->restricPermission = GetStringArrayTag(jsonObj, "restricted-permissions", &profVal->restricNum);
    return V_OK;
}

static int32_t GetProfDebugInfo(const cJSON *root, ProfDebugInfo *profVal)
{
    cJSON *jsonObj = cJSON_GetObjectItem(root, "debug-info");
    if (jsonObj == ((void*)0)) {
        ((void)HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify", "[%s:%d]: ""failed to get debug-info", __FUNCTION__, 204));
        return V_OK;
    }
    profVal->devIdType = GetStringTag(jsonObj, "device-id-type");
    if (profVal->devIdType == ((void*)0)) {
        ((void)HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify", "[%s:%d]: ""failed to get device-id-type", __FUNCTION__, 209));
        return V_OK;
    }
    profVal->deviceId = GetStringArrayTag(jsonObj, "device-ids", &profVal->devidNum);
    return V_OK;
}

static int32_t GetProfIssuerInfo(const cJSON *root, ProfileProf *pf)
{
    pf->issuer = GetStringTag(root, "issuer");
    if (pf->issuer == ((void*)0)) {
        int32_t len = strlen("Huawei App Store");
        pf->issuer = malloc(len + 1);
        if (pf->issuer == ((void*)0)) {
            return V_ERR;
        }
        errno_t ret = strcpy_s(pf->issuer, len + 1, "Huawei App Store");
        if (ret != 0) {
            do { if ((pf->issuer) != ((void*)0)) { free(pf->issuer); (pf->issuer) = ((void*)0); } } while (0);
            ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""str cpy error: %d", __FUNCTION__, 228, ret));
        }
        return ret;
    }
    return V_OK;
}

static void FreeProfBundle(ProfBundleInfo *pfval)
{
    do { do { if ((pfval->appFeature) != ((void*)0)) { free(pfval->appFeature); (pfval->appFeature) = ((void*)0); } } while (0); } while (0);
    do { do { if ((pfval->bundleName) != ((void*)0)) { free(pfval->bundleName); (pfval->bundleName) = ((void*)0); } } while (0); } while (0);
    do { do { if ((pfval->devCert) != ((void*)0)) { free(pfval->devCert); (pfval->devCert) = ((void*)0); } } while (0); } while (0);
    do { do { if ((pfval->developerId) != ((void*)0)) { free(pfval->developerId); (pfval->developerId) = ((void*)0); } } while (0); } while (0);
    do { do { if ((pfval->releaseCert) != ((void*)0)) { free(pfval->releaseCert); (pfval->releaseCert) = ((void*)0); } } while (0); } while (0);
    return;
}

static void FreeProfPerssion(ProfPermission *pfval)
{
    FreeStringAttay(pfval->permission, pfval->permissionNum);
    pfval->permissionNum = 0;
    pfval->permission = ((void*)0);

    FreeStringAttay(pfval->restricPermission, pfval->restricNum);
    pfval->restricNum = 0;
    pfval->restricPermission = ((void*)0);
    return;
}

static void FreeProfDebuginfo(ProfDebugInfo *pfval)
{
    do { do { if ((pfval->devIdType) != ((void*)0)) { free(pfval->devIdType); (pfval->devIdType) = ((void*)0); } } while (0); } while (0);

    FreeStringAttay(pfval->deviceId, pfval->devidNum);
    pfval->devidNum = 0;
    pfval->deviceId = ((void*)0);

    return;
}

void ProfFreeData(ProfileProf *pf)
{
    if (pf == ((void*)0)) {
        return;
    }
    do { do { if ((pf->versionName) != ((void*)0)) { free(pf->versionName); (pf->versionName) = ((void*)0); } } while (0); } while (0);
    do { do { if ((pf->uuid) != ((void*)0)) { free(pf->uuid); (pf->uuid) = ((void*)0); } } while (0); } while (0);
    do { do { if ((pf->type) != ((void*)0)) { free(pf->type); (pf->type) = ((void*)0); } } while (0); } while (0);
    do { do { if ((pf->appDistType) != ((void*)0)) { free(pf->appDistType); (pf->appDistType) = ((void*)0); } } while (0); } while (0);
    FreeProfBundle(&pf->bundleInfo);
    FreeProfPerssion(&pf->permission);
    FreeProfDebuginfo(&pf->debugInfo);
    do { do { if ((pf->issuer) != ((void*)0)) { free(pf->issuer); (pf->issuer) = ((void*)0); } } while (0); } while (0);
    do { do { if ((pf->appid) != ((void*)0)) { free(pf->appid); (pf->appid) = ((void*)0); } } while (0); } while (0);
    return;
}


int32_t ParseProfile(const char *buf, int32_t len, ProfileProf *pf)
{
    do { if ((pf) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""pf"" is null", __FUNCTION__, 288)); return V_ERR; } } while (0);
    do { if ((buf) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""buf"" is null", __FUNCTION__, 289)); return V_ERR; } } while (0);
    ProfInit(pf);
    int32_t ret;
    char *pfStr = strchr(buf, '{');
    do { if ((pfStr) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""pfStr"" is null", __FUNCTION__, 293)); return V_ERR; } } while (0);

    cJSON *root = cJSON_Parse(pfStr);
    do { if ((root) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""root"" is null", __FUNCTION__, 296)); return V_ERR; } } while (0);

    cJSON *jsonObj = cJSON_GetObjectItem(root, "version-code");
    do { if ((jsonObj) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""jsonObj"" is null", __FUNCTION__, 299)); goto EXIT; } } while (0);
    pf->versionCode = jsonObj->valueint;

    pf->versionName = GetStringTag(root, "version-name");
    do { if ((pf->versionName) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""pf->versionName"" is null", __FUNCTION__, 303)); goto EXIT; } } while (0);

    pf->uuid = GetStringTag(root, "uuid");
    do { if ((pf->uuid) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""pf->uuid"" is null", __FUNCTION__, 306)); goto EXIT; } } while (0);

    pf->type = GetStringTag(root, "type");
    do { if ((pf->type) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""pf->type"" is null", __FUNCTION__, 309)); goto EXIT; } } while (0);

    pf->appDistType = GetStringTag(root, "app-distribution-type");
    if (pf->appDistType == ((void*)0)) {
        pf->appDistType = malloc(sizeof(char));
        do { if ((pf->appDistType) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""pf->appDistType"" is null", __FUNCTION__, 314)); goto EXIT; } } while (0);
        pf->appDistType[0] = '\0';
    }

    ret = GetProfValidity(root, &pf->validity);
    do { if ((ret) != V_OK) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""ret"" not ok", __FUNCTION__, 319)); goto EXIT; } } while (0);

    ret = GetProfBundleInfo(root, &pf->bundleInfo);
    do { if ((ret) != V_OK) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""ret"" not ok", __FUNCTION__, 322)); goto EXIT; } } while (0);

    ret = GetProfPermission(root, &pf->permission);
    do { if ((ret) != V_OK) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""ret"" not ok", __FUNCTION__, 325)); goto EXIT; } } while (0);

    ret = GetProfDebugInfo(root, &pf->debugInfo);
    do { if ((ret) != V_OK) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""ret"" not ok", __FUNCTION__, 328)); goto EXIT; } } while (0);

    ret = GetProfIssuerInfo(root, pf);
    do { if ((ret) != V_OK) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""ret"" not ok", __FUNCTION__, 331)); goto EXIT; } } while (0);

    ((void)HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify", "[%s:%d]: ""parse profile json success", __FUNCTION__, 333));
    cJSON_Delete(root);
    return V_OK;

EXIT:
    cJSON_Delete(root);
    ProfFreeData(pf);
    return V_ERR;
}

static int32_t VerifyAppTypeAndDistribution(const ProfileProf *pf)
{
    if ((strcmp(pf->type, "debug") != 0) && (strcmp(pf->type, "release") != 0)) {
        ((void)HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify", "[%s:%d]: ""invalid app type: %s", __FUNCTION__, 346, pf->type));
        return V_ERR;
    }
    if (strcmp(pf->type, "release") == 0) {
        if ((strcmp(pf->appDistType, APP_GALLERY) != 0) && (strcmp(pf->appDistType, ENTERPRISE) != 0) &&
            (strcmp(pf->appDistType, ENTERPRISE_NORMAL) != 0) && (strcmp(pf->appDistType, ENTERPRISE_MDM) != 0) &&
            (strcmp(pf->appDistType, INTERNALTESTING) != 0) && (strcmp(pf->appDistType, OS_INTEGRATION) != 0)) {
            ((void)HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify", "[%s:%d]: ""invalid app dis type: %s", __FUNCTION__, 353, pf->appDistType));
            return V_ERR;
        }
    }
    return V_OK;
}

static int32_t VerifyAppBundleInfo(const ProfileProf *pf)
{
    if (strcmp(pf->type, "debug") == 0) {
        if (strlen((char *)pf->bundleInfo.devCert) == 0) {
            ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""debug app, dev cert null", __FUNCTION__, 364));
            return V_ERR;
        }
    } else if (strcmp(pf->type, "release") == 0) {
        if (strlen((char *)pf->bundleInfo.releaseCert) == 0) {
            ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""debug app, dev cert null", __FUNCTION__, 369));
            return V_ERR;
        }
    } else {
        ((void)HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify", "[%s:%d]: ""invalid app type: %s", __FUNCTION__, 373, pf->type));
        return V_ERR;
    }
    return V_OK;
}

static int32_t VerifyUdid(const ProfileProf *pf)
{
    uint32_t size = 64 + 1;
    if (pf->debugInfo.devidNum > 100) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""udid num exceed maximum", __FUNCTION__, 383));
        return V_ERR;
    }
    unsigned char *udid = malloc(size);
    if (udid == ((void*)0)) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""udid is null", __FUNCTION__, 388));
        return V_ERR;
    }
    (void)memset_s(udid, size, 0, size);
    int32_t result = InquiryDeviceUdid(udid, size);
    if (result != 0) {
        free(udid);
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""get udid fail, ret: %d", __FUNCTION__, 395, result));
        return V_ERR;
    }
    for (int32_t i = 0; i < pf->debugInfo.devidNum; i++) {
        if (strcmp((const char *)pf->debugInfo.deviceId[i], (const char *)udid) == 0) {
            ((void)HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify", "[%s:%d]: ""find right udid", __FUNCTION__, 400));
            free(udid);
            udid = ((void*)0);
            return V_OK;
        }
    }
    ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""udid invalid", __FUNCTION__, 406));
    free(udid);
    udid = ((void*)0);
    return V_ERR;
}

static int32_t VerifyDebugInfo(const ProfileProf *pf)
{
    if (strcmp(pf->type, "debug") != 0) {
        ((void)HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify", "[%s:%d]: ""not debug app, return ok", __FUNCTION__, 415));
        return V_OK;
    }
    ((void)HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify", "[%s:%d]: ""devid type: %s", __FUNCTION__, 418, pf->debugInfo.devIdType));
    int32_t ret;
    if (strcmp(pf->debugInfo.devIdType, "udid") == 0) {
        ret = VerifyUdid(pf);
    } else {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""devid type invalid", __FUNCTION__, 423));
        ret = V_ERR;
    }
    return ret;
}

int32_t VerifyProfileContent(const ProfileProf *pf)
{
    do { if ((pf) == ((void*)0)) { ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""pf"" is null", __FUNCTION__, 431)); return V_ERR; } } while (0);
    int32_t ret = VerifyAppTypeAndDistribution(pf);
    if (ret != V_OK) {
        ((void)HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify", "[%s:%d]: ""invalid profile distribution type : %s", __FUNCTION__, 434, pf->appDistType));
        return V_ERR_INVALID_DISP_TYPE;
    }
    ret = VerifyAppBundleInfo(pf);
    if (ret != V_OK) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""invalid profile app bundle info", __FUNCTION__, 439));
        return V_ERR_INVALID_APP_BUNDLE;
    }

    ret = VerifyDebugInfo(pf);
    if (ret != V_OK) {
        ((void)HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify", "[%s:%d]: ""validate debug info", __FUNCTION__, 445));
        return V_ERR_INVALID_DEVID;
    }
    return V_OK;
}
