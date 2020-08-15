use super::*;

packets! {
    EncryptionRequest {
        server_id String;
        public_key LengthPrefixedVec<u8>;
    }
}
