fn ParseSignerIssuerAndSerialNum(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo) -> i32 {
    use crate::types::*;
    
    let mut rc: i32;
    let mut len: size_t = 0;
    
    unsafe {
        rc = crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, (MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE) as i32);
        if rc != 0 {
            return rc;
        }
        
        (*signer).issuerRaw.p = *p;
        
        rc = crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, (MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE) as i32);
        if rc != 0 {
            return rc;
        }
        
        // mbedtls_x509_get_name is not available in compat.rs, so we need to manually parse
        // Skip over the name data by advancing p by len bytes
        let name_end = (*p).add(len as usize);
        *p = name_end;
        
        (*signer).issuerRaw.len = (*p).offset_from((*signer).issuerRaw.p) as size_t;
        
        // mbedtls_x509_get_serial is not available, parse manually
        // Get the serial number (INTEGER tag = 0x02)
        rc = crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, 0x02);
        if rc != 0 {
            return rc;
        }
        (*signer).serial.tag = 0x02;
        (*signer).serial.len = len;
        (*signer).serial.p = *p;
        *p = (*p).add(len as usize);
        
        0
    }
}