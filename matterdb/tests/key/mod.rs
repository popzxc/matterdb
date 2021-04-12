use matterdb::BinaryKey;

pub const HASH_SIZE: usize = 32;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Key(pub [u8; HASH_SIZE]);

impl From<[u8; HASH_SIZE]> for Key {
    fn from(key: [u8; HASH_SIZE]) -> Self {
        Self(key)
    }
}

impl BinaryKey for Key {
    fn size(&self) -> usize {
        HASH_SIZE
    }

    fn write(&self, buffer: &mut [u8]) -> usize {
        buffer.copy_from_slice(&self.0);
        self.0.len()
    }

    fn read(buffer: &[u8]) -> Self::Owned {
        let mut buf = [0; 32];
        buf.copy_from_slice(&buffer);
        Self(buf)
    }
}
