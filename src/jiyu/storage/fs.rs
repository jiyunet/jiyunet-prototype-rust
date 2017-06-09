
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Read;

use jiyu::storage::{Address, Entity, EntitySource};

pub struct LocalEntityStorage {
    root: PathBuf
}

impl LocalEntityStorage {

    fn new(path: PathBuf) -> Result<LocalEntityStorage, ()> {
        if path.is_dir() {
            Ok(LocalEntityStorage { root: path })
        } else {
            Err(())
        }
    }

}

impl<E> EntitySource<E> for LocalEntityStorage where E: Entity {

    fn load(&self, addr: Address) -> Result<Option<E>, ()> {

        let mut p = self.root.to_owned();
        let addr_str = addr.to_string();
        p.push(&addr_str[..4]);
        p.push(&addr_str[4..]);

        if p.exists() && p.is_file() {

            let mut f = File::open(p).unwrap();
            let mut buf = Vec::new();
            match f.read_to_end(buf.as_mut()) {
                Ok(v) => Ok(Some(E::from_bytes(buf.as_slice()))),
                Err(_) => Err(())
            }

        } else {
            Ok(None)
        }

    }

    fn store(&self, ent: E) {
        unimplemented!();
    }

}
