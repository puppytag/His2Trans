/*
 * Mock header for LiteOS build definitions
 * 
 * Placeholder for los_builddef.h which is LiteOS-specific
 */

#ifndef _LOS_BUILDDEF_H
#define _LOS_BUILDDEF_H

/* Common LiteOS type definitions (placeholders) */
typedef unsigned char UINT8;
typedef unsigned short UINT16;
typedef unsigned int UINT32;
typedef unsigned long long UINT64;
typedef signed char INT8;
typedef signed short INT16;
typedef signed int INT32;
typedef signed long long INT64;
typedef void VOID;
typedef char CHAR;
typedef int BOOL;

#define TRUE 1
#define FALSE 0

#define LITE_OS_SEC_TEXT
#define LITE_OS_SEC_TEXT_INIT
#define LITE_OS_SEC_TEXT_MINOR
#define LITE_OS_SEC_DATA
#define LITE_OS_SEC_BSS

#endif /* _LOS_BUILDDEF_H */
