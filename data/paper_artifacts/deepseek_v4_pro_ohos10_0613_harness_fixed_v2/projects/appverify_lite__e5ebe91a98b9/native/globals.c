/* Shim C file to provide global variable definitions for Rust crate.
   These variables were originally defined in various .c files and are now
   provided here so that Rust can import them via extern "C". */

#include <stddef.h>

/* ---------------------------------------------------------------- */
/* Minimal type definitions matching bindgen layout (same ABI)       */
/* ---------------------------------------------------------------- */

/* TrustAppCert from src/app_verify.c */
typedef struct {
    int          maxCertPath;
    const char  *name;
    const char  *appSignCert;
    const char  *profileSignCert;
    const char  *profileDebugSignCert;
    const char  *issueCA;
} TrustAppCert;

#define CERT_MAX_DEPTH 3

/* ProductDiff / GetDeviceUdid from src/app_verify_hal.c */
typedef int (*GetDeviceUdid)(unsigned char *udid, int size);

typedef struct {
    GetDeviceUdid devUdidFunc;
} ProductDiff;

/* ---------------------------------------------------------------- */
/* 1. Variables from app_verify.c                                   */
/* ---------------------------------------------------------------- */

const TrustAppCert g_trustAppList[3] = {
    {
        .maxCertPath = CERT_MAX_DEPTH,
        .name = "huawei app gallary",
        .appSignCert = "C=CN, O=Huawei, OU=HOS AppGallery, CN=HOS AppGallery Application Release",
        .profileSignCert = "C=CN, O=Huawei, OU=HOS AppGallery, CN=HOS Profile Management",
        .profileDebugSignCert = "C=CN, O=Huawei, OU=HOS AppGallery, CN=HOS Profile Management Debug",
        .issueCA = "C=CN, O=Huawei, OU=Huawei CBG, CN=Huawei CBG Software Signing Service CA",
    },
    {
        .maxCertPath = CERT_MAX_DEPTH,
        .name = "huawei system apps",
        .appSignCert = "C=CN, O=Huawei CBG, OU=HOS Development Team, CN=HOS Application Provision Release",
        .profileSignCert = "C=CN, O=Huawei CBG, OU=HOS Development Team, CN=HOS Application Provision Profile Release",
        .profileDebugSignCert = "C=CN, O=Huawei CBG, OU=HOS Development Team, CN=HOS Application Provision Profile Release_Debug",
        .issueCA = "C=CN, O=Huawei, OU=Huawei CBG, CN=Huawei CBG Software Signing Service CA",
    },
#ifndef OHOS_SIGN_HAPS_BY_SERVER
    {
        .maxCertPath = CERT_MAX_DEPTH,
        .name = "OpenHarmony apps",
        .appSignCert = "C=CN, O=OpenHarmony, OU=OpenHarmony Team, CN=OpenHarmony Application Release",
        .profileSignCert = "C=CN, O=OpenHarmony, OU=OpenHarmony Team, CN=OpenHarmony Application Profile Release",
        .profileDebugSignCert = "C=CN, O=OpenHarmony, OU=OpenHarmony Team, CN=OpenHarmony Application Profile Debug",
        .issueCA = "C=CN, O=OpenHarmony, OU=OpenHarmony Team, CN=OpenHarmony Application CA",
    },
#endif
};

const TrustAppCert g_trustAppListTest[2] = {
    {
        .maxCertPath = CERT_MAX_DEPTH,
        .name = "huawei app gallary",
        .appSignCert = "C=CN, O=Huawei, OU=HOS AppGallery, CN=HOS AppGallery Application Release",
        .profileSignCert = "C=CN, O=Huawei, OU=HOS AppGallery, CN=HOS Profile Management",
        .profileDebugSignCert = "C=CN, O=Huawei, OU=HOS AppGallery, CN=HOS Profile Management Debug",
        .issueCA = "C=CN, O=Huawei, OU=Huawei CBG, CN=Huawei CBG Software Signing Service CA Test",
    },
    {
        .maxCertPath = CERT_MAX_DEPTH,
        .name = "huawei system apps",
        .appSignCert = "C=CN, O=Huawei CBG, OU=HOS Development Team, CN=HOS Application Provision Dev",
        .profileSignCert = "C=CN, O=Huawei CBG, OU=HOS Development Team, CN=HOS Application Provision Profile Dev",
        .profileDebugSignCert = "C=CN, O=Huawei CBG, OU=HOS Development Team, CN=HOS Application Provision Profile Dev_Debug",
        .issueCA = "C=CN, O=Huawei, OU=Huawei CBG, CN=Huawei CBG Software Signing Service CA Test",
    },
};

/* ---------------------------------------------------------------- */
/* 2. Variable from app_verify_hal.c                                */
/* ---------------------------------------------------------------- */

/* Initialized by RegistHalFunc at runtime; start zero-initialized. */
ProductDiff g_productDiffFunc = { NULL };

/* ---------------------------------------------------------------- */
/* 3. Variables from mbedtls_pkcs7.c                                */
/* ---------------------------------------------------------------- */

/* PEM certificate bytes – originally static const arrays.
   We define them as mutable so that Rust's static mut can link to them.
   (Rust declaration is `static mut`, so the symbol must be writable.) */

#define DEBUG_MODE_ROOT_CERT_IN_PEM_SIZE 821
unsigned char DEBUG_MODE_ROOT_CERT_IN_PEM[DEBUG_MODE_ROOT_CERT_IN_PEM_SIZE];

#define OHOS_ROOT_CERT_IN_PEM_SIZE 863
unsigned char OHOS_ROOT_CERT_IN_PEM[OHOS_ROOT_CERT_IN_PEM_SIZE];

#define ROOT_CA_G2_CERT_IN_PEM_SIZE 805
unsigned char ROOT_CA_G2_CERT_IN_PEM[ROOT_CA_G2_CERT_IN_PEM_SIZE];

/* mbedtls_x509_crt structures – large C types.  To avoid pulling in the
   full mbedtls header tree we allocate enough raw bytes (size derived from
   the bindgen Rust struct, which is 1064 bytes).  We align to 8 because
   the struct contains pointers and 64-bit fields. */
#define MBEDTLS_X509_CRT_SZ 1064

__attribute__((aligned(8))) unsigned char g_debugModeRootCert[MBEDTLS_X509_CRT_SZ];
__attribute__((aligned(8))) unsigned char g_ohosRootCert[MBEDTLS_X509_CRT_SZ];
__attribute__((aligned(8))) unsigned char g_rootCaG2Cert[MBEDTLS_X509_CRT_SZ];

