/*
 * Mock header file for bindgen
 * 
 * This is a placeholder for bits/c++config.h to allow bindgen to work
 * without requiring the full C++ STL headers.
 * 
 * Since we only need skeleton generation (not actual C++ STL functionality),
 * this empty mock is sufficient.
 */

#ifndef _GLIBCXX_CXX_CONFIG_H
#define _GLIBCXX_CXX_CONFIG_H

/* Define __GLIBC_PREREQ to avoid macro expansion errors */
#ifndef __GLIBC_PREREQ
#define __GLIBC_PREREQ(x, y) 0
#endif

/* Basic definitions that might be expected */
#define _GLIBCXX_BEGIN_NAMESPACE_VERSION
#define _GLIBCXX_END_NAMESPACE_VERSION

#endif /* _GLIBCXX_CXX_CONFIG_H */
