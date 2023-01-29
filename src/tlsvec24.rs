use std::io::{Read, Write};
use tls_codec::{Deserialize, Error, Serialize, Size};

#[derive(Clone, PartialEq, Eq, Debug)]
struct U24 {
    data: usize,
}

impl U24 {
    const MAX: usize = 16_777_215;
}

impl Serialize for U24 {
    fn tls_serialize<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        let buf: [u8; 3] = [
            (self.data >> 16) as u8,
            (self.data >> 8) as u8,
            self.data as u8,
        ];
        writer.write_all(&buf)?;
        Ok(buf.len())
    }
}

impl Size for U24 {
    #[inline]
    fn tls_serialized_len(&self) -> usize {
        3
    }
}

impl Deserialize for U24 {
    fn tls_deserialize<R: Read>(bytes: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 3];
        bytes.read_exact(&mut buf)?;
        Ok(U24 {
            data: (buf[0] as usize) << 16 | (buf[1] as usize) << 8 | (buf[2] as usize),
        })
    }
}

#[test]
fn roundtrip() {
    let values = [0, 1, 255, 256, 50000, U24::MAX];
    for d in values {
        let ser = U24 { data: d };
        let mut buf = Vec::new();
        ser.tls_serialize(&mut buf).expect("serializes");
        let de = U24::tls_deserialize(&mut buf.as_slice()).expect("deserializes");
        assert_eq!(ser.data, de.data);
    }
}

//impl_tls_vec_generic!(U24, TlsVec24, 3);
#[derive(Eq, Debug)]
pub struct TlsVec24<T> {
    vec: Vec<T>,
}

impl<T: Clone> Clone for TlsVec24<T> {
    fn clone(&self) -> Self {
        Self::new(self.vec.clone())
    }
}

impl<T: core::hash::Hash> core::hash::Hash for TlsVec24<T> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.vec.hash(state)
    }
}

impl<T> core::ops::Index<usize> for TlsVec24<T> {
    type Output = T;

    #[inline]
    fn index(&self, i: usize) -> &T {
        self.vec.index(i)
    }
}

impl<T: core::cmp::PartialEq> core::cmp::PartialEq for TlsVec24<T> {
    fn eq(&self, other: &Self) -> bool {
        self.vec.eq(&other.vec)
    }
}

impl<T> core::ops::IndexMut<usize> for TlsVec24<T> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.vec.index_mut(i)
    }
}

impl<T> core::borrow::Borrow<[T]> for TlsVec24<T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        &self.vec
    }
}

impl<T> core::iter::FromIterator<T> for TlsVec24<T> {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let vec = Vec::<T>::from_iter(iter);
        Self { vec }
    }
}

impl<T> From<Vec<T>> for TlsVec24<T> {
    #[inline]
    fn from(v: Vec<T>) -> Self {
        Self::new(v)
    }
}

impl<T: Clone> From<&[T]> for TlsVec24<T> {
    #[inline]
    fn from(v: &[T]) -> Self {
        Self::from_slice(v)
    }
}

impl<T> From<TlsVec24<T>> for Vec<T> {
    #[inline]
    fn from(mut v: TlsVec24<T>) -> Self {
        core::mem::take(&mut v.vec)
    }
}

impl<T> Default for TlsVec24<T> {
    #[inline]
    fn default() -> Self {
        Self { vec: Vec::new() }
    }
}

//impl_tls_vec_codec_generic!(U24, TlsVec24, 3); {
impl<T: Serialize> Serialize for TlsVec24<T> {
    fn tls_serialize<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        self.serialize(writer)
    }
}

impl<T: Size> Size for TlsVec24<T> {
    #[inline]
    fn tls_serialized_len(&self) -> usize {
        self.tls_serialized_length()
    }
}

impl<T: Serialize> Serialize for &TlsVec24<T> {
    fn tls_serialize<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        self.serialize(writer)
    }
}

impl<T: Size> Size for &TlsVec24<T> {
    #[inline]
    fn tls_serialized_len(&self) -> usize {
        self.tls_serialized_length()
    }
}

impl<T: Deserialize> Deserialize for TlsVec24<T> {
    fn tls_deserialize<R: Read>(bytes: &mut R) -> Result<Self, Error> {
        Self::deserialize(bytes)
    }
}
// }

impl<T: Serialize> TlsVec24<T> {
    // impl_serialize!(self, U24, TlsVec24, 3);
    #[inline(always)]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        // Get the byte length of the content, make sure it's not too
        // large and write it out.
        let tls_serialized_len = self.tls_serialized_len();
        let byte_length = tls_serialized_len - 3;

        debug_assert!(
            byte_length <= U24::MAX,
            "Vector length can't be encoded in the vector length a {} >= {}",
            byte_length,
            U24::MAX
        );
        if byte_length > U24::MAX {
            return Err(Error::InvalidVectorLength);
        }

        let mut written = U24 { data: byte_length }.tls_serialize(writer)?;

        // Now serialize the elements
        for e in self.as_slice().iter() {
            written += e.tls_serialize(writer)?;
        }

        debug_assert_eq!(
            written, tls_serialized_len,
            "{} bytes should have been serialized but {} were written",
            tls_serialized_len, written
        );
        if written != tls_serialized_len {
            return Err(Error::EncodingError(format!(
                "{} bytes should have been serialized but {} were written",
                tls_serialized_len, written
            )));
        }
        Ok(written)
    }
    //
}

impl<T: Size> TlsVec24<T> {
    #[inline(always)]
    fn tls_serialized_length(&self) -> usize {
        self.as_slice()
            .iter()
            .fold(3, |acc, e| acc + e.tls_serialized_len())
    }
}

impl<T: Deserialize> TlsVec24<T> {
    #[inline(always)]
    fn deserialize<R: Read>(bytes: &mut R) -> Result<Self, Error> {
        let mut result = Self { vec: Vec::new() };
        let len = <U24>::tls_deserialize(bytes)?;
        let mut read = len.tls_serialized_len();
        let len_len = read;
        while (read - len_len) < len.data {
            let element = T::tls_deserialize(bytes)?;
            read += element.tls_serialized_len();
            result.push(element);
        }
        Ok(result)
    }
}

impl<T> TlsVec24<T> {
    /// Create a new `TlsVec` from a Rust Vec.
    #[inline]
    pub fn new(vec: Vec<T>) -> Self {
        Self { vec }
    }

    /// Create a new `TlsVec` from a slice.
    #[inline]
    pub fn from_slice(slice: &[T]) -> Self
    where
        T: Clone,
    {
        Self {
            vec: slice.to_vec(),
        }
    }

    /// Get the length of the vector.
    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Get a slice to the raw vector.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.vec
    }

    /// Check if the vector is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// Get the underlying vector and consume this.
    #[inline]
    pub fn into_vec(mut self) -> Vec<T> {
        core::mem::take(&mut self.vec)
    }

    /// Add an element to this.
    #[inline]
    pub fn push(&mut self, value: T) {
        self.vec.push(value);
    }

    /// Remove the last element.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    /// Remove the element at `index`.
    #[inline]
    pub fn remove(&mut self, index: usize) -> T {
        self.vec.remove(index)
    }

    /// Returns a reference to an element or subslice depending on the type of index.
    /// XXX: implement SliceIndex instead
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index)
    }

    /// Returns an iterator over the slice.
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.vec.iter()
    }

    /// Retains only the elements specified by the predicate.
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.vec.retain(f)
    }

    /// Get the number of bytes used for the length encoding.
    #[inline(always)]
    pub fn len_len() -> usize {
        3
    }
}
