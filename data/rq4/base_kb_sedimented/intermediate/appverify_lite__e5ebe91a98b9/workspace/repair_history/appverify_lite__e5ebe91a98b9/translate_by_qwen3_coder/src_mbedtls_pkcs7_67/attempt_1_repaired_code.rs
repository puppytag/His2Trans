pub struct Pkcs7 {
    pub contentTypeOid: mbedtls_asn1_buf,
    pub signedData: SignedData,
}