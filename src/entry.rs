
use tls_codec_derive::{TlsSerialize, TlsDeserialize, TlsSize};

#[derive(TlsSerialize, TlsDeserialize, TlsSize, PartialEq, Debug, Clone)]
// MerkleTreeLeaf is described in S3.4.
struct MerkleTreeLeaf {
    version: u8,
    signed_entry: MerkleLeaf,
}

#[derive(TlsSerialize, TlsDeserialize, TlsSize, PartialEq, Debug, Clone)]
#[repr(u8)]
enum MerkleLeaf {
    #[tls_codec(discriminant=0)]
    TimeStampedEntry(TimeStampedEntry)
}

#[derive(TlsSerialize, TlsDeserialize, TlsSize, PartialEq, Debug, Clone)]
// TimeStampedEntry is described in S3.4.
struct TimeStampedEntry {
    timestamp: u64,
    log_entry: LogEntry,
    extensions: tls_codec::TlsByteVecU16,
}

// LogEntry is the LogEntryType and signed_entry
// It occurs in both the SignedCertificateTimestamp and TimeStampedEntry
#[derive(TlsSerialize, TlsDeserialize, TlsSize, PartialEq, Debug, Clone)]
#[repr(u16)]
enum LogEntry { 
    #[tls_codec(discriminant=0)]
    X509Entry(Asn1Cert), 
    #[tls_codec(discriminant=1)]
    PrecertEntry(PreCert),
}

#[derive(TlsSerialize, TlsDeserialize, TlsSize, PartialEq, Debug, Clone)]
struct Asn1Cert {
    // TODO:  Limit length to 24 bits
    opaque: tls_codec::TlsByteVecU32
}

// PreCert is defined in S3.2. 
#[derive(TlsSerialize, TlsDeserialize, TlsSize, PartialEq, Debug, Clone)]
struct PreCert {
    issuer_key_hash: [u8; 32],
    // TODO: Limit length to 24 bits
    tbs_certificate: tls_codec::TlsByteVecU32,
}

#[test]
fn test() {
    use tls_codec::Deserialize;

    let mut x509 = &[1u8, 0, 1, 2, 3, 4, 5, 6, 7, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] as &[u8];
    let v = MerkleTreeLeaf::tls_deserialize(&mut x509).expect("deserialize");
    assert_eq!(v, MerkleTreeLeaf{
        version: 1,
        signed_entry: MerkleLeaf::TimeStampedEntry(TimeStampedEntry {
            timestamp: 72623859790383103,
            log_entry: LogEntry::X509Entry(Asn1Cert{
                opaque: vec![].into(),
            }),
            extensions: vec![].into(),
        })
    });
}