use std::hash::{BuildHasher, Hasher};

pub(crate) struct NoopHasher {
    hash: u64
}

impl Default for NoopHasher {
    fn default() -> Self {
        Self {
            hash: 0
        }
    }
}

impl BuildHasher for NoopHasher {
    type Hasher = NoopHasher;

    fn build_hasher(&self) -> Self::Hasher {
        NoopHasher::default()
    }
}

impl Hasher for NoopHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        // Key is supposed to be hash already. we don't want to hash the hash.
        self.hash = u64::from_ne_bytes(bytes.try_into().unwrap());
    }
}
