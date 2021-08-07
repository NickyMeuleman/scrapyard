use std::borrow::Borrow;
use std::io::{Read, Result, Write};

type KeyIter<'a> = std::iter::Cycle<std::slice::Iter<'a, u8>>;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: KeyIter<'a>,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<KeySource: ?Sized + AsRef<[u8]>>(key: &'a KeySource) -> Self {
        Self {
            key: key.as_ref().iter().cycle(),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut()
            .for_each(|num| *num ^= self.key.next().expect("infinite iterator"));
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    // the 'a lifetime refers to the lifetime of the key argument at the time of a call to Xorcism::new(key)
    // the 'b lifetime here refers to the lifetime of the created Xorcism struct. That can be shorter than 'a
    // the returned iterator is dependent on both of those lifetimes
    // because the impl syntax doesn't support this we use an empty trait with a lifetime
    // my question: why is using only 'b not enough? It will always be shorter than 'a
    // guess at answer: the compiler can't figure that out
    pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + 'b + Captures<'a>
    where
        Data: IntoIterator,
        Data::Item: Borrow<u8>,
        Data::IntoIter: 'b,
    {
        data.into_iter()
            .map(move |item| item.borrow() ^ self.key.next().expect("infinite iterator"))
    }

    pub fn reader<R: Read>(self, reader: R) -> XorReader<'a, R> {
        XorReader {
            xorcism: self,
            reader,
        }
    }

    pub fn writer<W: Write>(self, writer: W) -> XorWriter<'a, W> {
        XorWriter {
            xorcism: self,
            writer,
        }
    }
}

pub struct XorReader<'a, R: Read> {
    xorcism: Xorcism<'a>,
    reader: R,
}

impl<'a, R: Read> Read for XorReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let count = self.reader.read(buf)?;
        self.xorcism.munge_in_place(buf);
        Ok(count)
    }
}

pub struct XorWriter<'a, W: Write> {
    xorcism: Xorcism<'a>,
    writer: W,
}

impl<'a, W: Write> Write for XorWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let munged: Vec<u8> = self.xorcism.munge(buf).collect();
        self.writer.write(&munged)
    }
    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

// Workaround for E0700 due to `impl Trait` not being able to have multiple lifetimes
// See: https://stackoverflow.com/a/50548538
pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

// Second option I don't fully understand with a seperate implementation with a key iterating over &u8
// use std::borrow::Borrow;
// use std::io::{Read, Result, Write};

// type KeyIter<'a> = std::iter::Cycle<std::slice::Iter<'a, u8>>;

// /// A munger which XORs a key with some data
// #[derive(Clone)]
// pub struct Xorcism<Key> {
//     key: Key,
// }

// impl<'a> Xorcism<KeyIter<'a>> {
//     /// Create a new Xorcism munger from a key
//     ///
//     /// Should accept anything which has a cheap conversion to a byte slice.
//     pub fn new<KeySource: ?Sized + AsRef<[u8]>>(key: &'a KeySource) -> Self {
//         Self {
//             key: key.as_ref().iter().cycle(),
//         }
//     }

//     /// XOR each byte of the input buffer with a byte from the key.
//     ///
//     /// Note that this is stateful: repeated calls are likely to produce different results,
//     /// even with identical inputs.
//     pub fn munge_in_place(&mut self, data: &mut [u8]) {
//         data.iter_mut()
//             .for_each(|num| *num ^= self.key.next().expect("infinite iterator"));
//     }

//     pub fn reader<R: Read>(self, reader: R) -> XorReader<'a, R> {
//         XorReader {
//             xorcism: self,
//             reader,
//         }
//     }

//     pub fn writer<W: Write>(self, writer: W) -> XorWriter<'a, W> {
//         XorWriter {
//             xorcism: self,
//             writer,
//         }
//     }
// }

// // by looking at midnightexigent's solution
// // implementation for the Xorcism struct for an iterator over references to bytes
// // I still don't really understand how this works, lifetimes are hard
// // where munge() has a seperate lifetime for self, so you don't have to collect() into a vec only to call into_iter() on it
// impl<'a, Key: Iterator<Item = &'a u8>> Xorcism<Key> {
//     /// XOR each byte of the data with a byte from the key.
//     ///
//     /// Note that this is stateful: repeated calls are likely to produce different results,
//     /// even with identical inputs.
//     ///
//     /// Should accept anything which has a cheap conversion to a byte iterator.
//     /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
//     pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + 'b
//     where
//         Data: IntoIterator,
//         Data::Item: Borrow<u8>,
//         Data::IntoIter: 'b,
//     {
//         // should the expect call here not happen? Since the impl is not for a infinite iterator?
//         data.into_iter()
//             .map(move |item| item.borrow() ^ self.key.next().expect("infinite iterator"))
//     }
// }

// pub struct XorReader<'a, R: Read> {
//     xorcism: Xorcism<KeyIter<'a>>,
//     reader: R,
// }

// impl<'a, R: Read> Read for XorReader<'a, R> {
//     fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
//         let count = self.reader.read(buf)?;
//         self.xorcism.munge_in_place(buf);
//         Ok(count)
//     }
// }

// pub struct XorWriter<'a, W: Write> {
//     xorcism: Xorcism<KeyIter<'a>>,
//     writer: W,
// }

// impl<'a, W: Write> Write for XorWriter<'a, W> {
//     fn write(&mut self, buf: &[u8]) -> Result<usize> {
//         let munged: Vec<u8> = self.xorcism.munge(buf).collect();
//         self.writer.write(&munged)
//     }
//     fn flush(&mut self) -> Result<()> {
//         self.writer.flush()
//     }
// }