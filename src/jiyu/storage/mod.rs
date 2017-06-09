#![allow(unused_imports, dead_code, unused_variables)]

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Read;
use std::fmt;

mod fs;

pub struct Address {
    id: [u8; 32] // FIXME Make this 32 configuratble based on which hash algo we choose.
}

impl Address {

    pub fn from_str(hex: &str) -> Address {
        unimplemented!()
    }

    pub fn to_string(&self) -> String {

        let mut hex = String::with_capacity(64);

        for b in self.id.into_iter() {

            let conv = |n| match n {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                10 => 'a',
                11 => 'b',
                12 => 'c',
                13 => 'd',
                14 => 'e',
                _ => 'f' // Shouldn't be anything other than 15.
            };

            hex.push(conv((b & 0xf0) >> 4));
            hex.push(conv(b & 0x0f));

        }

        assert_eq!(hex.len(), 64);
        hex.to_owned()

    }

}

pub trait Entity {
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(&self) -> &[u8];
}

pub trait EntitySource<E> where E: Entity {
    fn load(&self, addr: Address) -> Result<Option<E>, ()>;
    fn store(&self, ent: E);
}
