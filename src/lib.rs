// TODO(nicholasbishop): remove this
#![allow(dead_code)]

mod key;
mod rangeset;

use key::Key;
use rangeset::{Range, RangeSet};
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

    vert_range_set: RangeSet,
    edge_range_set: RangeSet
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            verts: HashMap::new(),
            edges: HashMap::new(),
            loops: HashMap::new(),
            faces: HashMap::new(),

            // TODO
            vert_range_set: RangeSet::new(Range::new(0, 0xffffffff - 1)),
            edge_range_set: RangeSet::new(Range::new(0, 0xffffffff - 1))
        }
    }

    pub fn add_vert(&mut self) -> Option<VKey> {
        if let Some(val) = self.vert_range_set.take_any_one() {
            let vkey = VKey::new(val);
            self.verts.insert(vkey, Vert { edges: Vec::new() });
            Some(vkey)
        } else {
            None
        }
    }

    pub fn add_edge(&mut self, v0: VKey, v1: VKey) -> Option<EKey> {
        // TODO(nicholasbishop): check that the verts are in the mesh,
        // are not the same vert, and that the edge doesn't exist yet.
        if let Some(val) = self.edge_range_set.take_any_one() {
            let ekey = EKey::new(val);
            self.verts.get_mut(&v0).unwrap().edges.push(ekey);
            self.verts.get_mut(&v1).unwrap().edges.push(ekey);
            self.edges.insert(ekey, Edge { verts: [v0, v1] });
            Some(ekey)
        } else {
            None
        }
    }
}

pub struct Vert {
    edges: Vec<EKey>
}

pub struct Edge {
    verts: [VKey; 2],
    // faces: Vec<FKey>
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_vert() {
        let mut mesh = Mesh::new();
        assert!(mesh.add_vert().is_some());
        assert!(!mesh.verts.is_empty());
    }

    #[test]
    fn test_add_edge() {
        let mut mesh = Mesh::new();
        let a = mesh.add_vert().unwrap();
        let b = mesh.add_vert().unwrap();
        let e = mesh.add_edge(a, b).unwrap();
        assert!(mesh.verts[&a].edges.contains(&e));
        assert!(mesh.verts[&b].edges.contains(&e));
    }
}
