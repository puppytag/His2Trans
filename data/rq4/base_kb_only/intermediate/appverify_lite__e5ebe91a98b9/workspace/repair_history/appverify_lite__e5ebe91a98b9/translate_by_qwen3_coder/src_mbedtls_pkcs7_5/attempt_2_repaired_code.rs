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
        
        let name_end = (*p).add(len as usize);
        while *p < name_end as *mut u8 {
            let mut set_len: size_t = 0;
            rc = crate::compat::mbedtls_asn1_get_tag(p, name_end, &mut set_len, (MBEDTLS_ASN1_CONSTRUCTED | 0x11) as i32);
            if rc != 0 {
                return rc;
            }
            *p = (*p).add(set_len as usize);
        }
        
        (*signer).issuerRaw.len = (*p).offset_from((*signer).issuerRaw.p) as size_t;
        
        let mut serial_len: size_t = 0;
        rc = crate::compat::mbedtls_asn1_get_tag(p, end, &mut serial_len, 0x02);
        if rc != 0 {
            return rc;
        }
        (*signer).serial.tag = 0x02;
        (*signer).serial.len = serial_len;
        (*signer).serial.p = *p;
        *p = (*p).add(serial_len as usize);
        
        0
    }
}