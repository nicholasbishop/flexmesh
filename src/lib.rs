// TODO(nicholasbishop): remove this
#![allow(dead_code)]

mod key;
mod rangeset;

use key::Key;
use std::collections::HashMap;

pub type VKey = Key;
pub type EKey = Key;
pub type FKey = Key;
pub type LKey = Key;
pub type FaceLen = u32;

pub struct Mesh {
    verts: HashMap<VKey, Vert>,
    edges: HashMap<EKey, Edge>,
    loops: HashMap<LKey, Loop>,
    faces: HashMap<FKey, Face>,

    // TODO
    next_vkey: u32
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            verts: HashMap::new(),
            edges: HashMap::new(),
            loops: HashMap::new(),
            faces: HashMap::new(),

            // TODO
            next_vkey: 0
        }
    }

    pub fn add_vert(&mut self) -> VKey {
        let v = Vert { edge: Key::invalid() };
        let vkey = VKey::new(self.next_vkey);
        self.verts.insert(vkey, v);
        self.next_vkey += 1;
        vkey
    }
}

pub struct Vert {
    edge: EKey
}

pub struct Edge {
    verts: [VKey; 2],
    faces: Vec<FKey>
}

pub struct Loop {
    next: LKey,
    vert: VKey,
    edge: EKey,
    face: FKey
}

pub struct Face {
    len: FaceLen,
    first_loop: LKey
}

#[test]
fn it_works() {
}
