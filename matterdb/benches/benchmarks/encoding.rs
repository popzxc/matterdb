use std::{borrow::Cow, fmt::Debug};

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use criterion::{black_box, Bencher, Criterion};
use rand::{rngs::StdRng, RngCore, SeedableRng};

use matterdb::BinaryValue;

const CHUNK_SIZE: usize = 64;
const SEED: [u8; 32] = [100; 32];

#[derive(Debug, Clone, Copy, PartialEq)]
struct SimpleData {
    id: u16,
    class: i16,
    value: i32,
}

impl BinaryValue for SimpleData {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = vec![0; 8];
        LittleEndian::write_u16(&mut buffer[0..2], self.id);
        LittleEndian::write_i16(&mut buffer[2..4], self.class);
        LittleEndian::write_i32(&mut buffer[4..8], self.value);
        buffer
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> anyhow::Result<Self> {
        let bytes = bytes.as_ref();
        let id = LittleEndian::read_u16(&bytes[0..2]);
        let class = LittleEndian::read_i16(&bytes[2..4]);
        let value = LittleEndian::read_i32(&bytes[4..8]);
        Ok(Self { id, class, value })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct CursorData {
    id: u16,
    class: i16,
    value: i32,
}

impl BinaryValue for CursorData {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; 8];
        let mut cursor = buf.as_mut_slice();
        cursor.write_u16::<LittleEndian>(self.id).unwrap();
        cursor.write_i16::<LittleEndian>(self.class).unwrap();
        cursor.write_i32::<LittleEndian>(self.value).unwrap();
        buf
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> anyhow::Result<Self> {
        let mut cursor = bytes.as_ref();
        let id = cursor.read_u16::<LittleEndian>()?;
        let class = cursor.read_i16::<LittleEndian>()?;
        let value = cursor.read_i32::<LittleEndian>()?;
        Ok(Self { id, class, value })
    }
}

fn gen_bytes_data() -> Vec<u8> {
    let mut rng: StdRng = SeedableRng::from_seed(SEED);
    let mut v = vec![0; CHUNK_SIZE];
    rng.fill_bytes(&mut v);
    v
}

fn check_binary_value<T>(data: T) -> T
where
    T: BinaryValue + Debug + PartialEq,
{
    let bytes = data.to_bytes();
    assert_eq!(T::from_bytes(bytes.into()).unwrap(), data);
    data
}

fn gen_sample_data() -> SimpleData {
    check_binary_value(SimpleData {
        id: 1,
        class: -5,
        value: 2127,
    })
}

fn gen_cursor_data() -> CursorData {
    check_binary_value(CursorData {
        id: 1,
        class: -5,
        value: 2127,
    })
}

fn bench_binary_value<F, V>(c: &mut Criterion, name: &str, f: F)
where
    F: Fn() -> V + 'static + Clone + Copy,
    V: BinaryValue + PartialEq + Debug,
{
    // Checks that binary value is correct.
    let val = f();
    let bytes = val.to_bytes();
    let val2 = V::from_bytes(bytes.into()).unwrap();
    assert_eq!(val, val2);
    // Runs benchmarks.
    c.bench_function(
        &format!("encoding/{}/to_bytes", name),
        move |b: &mut Bencher<'_>| {
            b.iter_with_setup(f, |data| black_box(data.to_bytes()));
        },
    );
    c.bench_function(
        &format!("encoding/{}/into_bytes", name),
        move |b: &mut Bencher<'_>| {
            b.iter_with_setup(f, |data| black_box(data.into_bytes()));
        },
    );
    c.bench_function(
        &format!("encoding/{}/from_bytes", name),
        move |b: &mut Bencher<'_>| {
            b.iter_with_setup(
                || {
                    let val = f();
                    val.to_bytes().into()
                },
                |bytes| black_box(V::from_bytes(bytes).unwrap()),
            );
        },
    );
}

pub fn bench_encoding(c: &mut Criterion) {
    bench_binary_value(c, "bytes", gen_bytes_data);
    bench_binary_value(c, "simple", gen_sample_data);
    bench_binary_value(c, "cursor", gen_cursor_data);
}
