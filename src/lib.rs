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
    edge_range_set: RangeSet,
    face_range_set: RangeSet
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
            edge_range_set: RangeSet::new(Range::new(0, 0xffffffff - 1)),
            face_range_set: RangeSet::new(Range::new(0, 0xffffffff - 1))
        }
    }

    /// Return the edge between two verts, or None if no such edge
    /// exists.
    pub fn find_edge(&self, vk0: VKey, vk1: VKey) -> Option<EKey> {
        for ek in self.verts[&vk0].edges.iter() {
            if self.edges[ek].is_vert_adjacent(vk1) {
                return Some(*ek)
            }
        }
        None
    }

    /// Add a new isolated vertex and return its key. Fail and return
    /// None if there are no more vertex keys available.
    pub fn add_vert(&mut self) -> Option<VKey> {
        if let Some(val) = self.vert_range_set.take_any_one() {
            let vkey = VKey::new(val);
            self.verts.insert(vkey, Vert::new());
            Some(vkey)
        } else {
            None
        }
    }

    pub fn add_edge(&mut self, vk0: VKey, vk1: VKey) -> Option<EKey> {
        // TODO(nicholasbishop): check that the verts are in the mesh,
        if vk0 == vk1 {
            None
        } else if let Some(ek) = self.find_edge(vk0, vk1) {
            Some(ek)
        } else {
            if let Some(val) = self.edge_range_set.take_any_one() {
                let ekey = EKey::new(val);
                self.verts.get_mut(&vk0).unwrap().edges.push(ekey);
                self.verts.get_mut(&vk1).unwrap().edges.push(ekey);
                self.edges.insert(ekey, Edge { verts: [vk0, vk1] });
                Some(ekey)
            } else {
                None
            }
        }
    }

    pub fn add_face(&mut self, vk: &[VKey]) -> Option<FKey> {
        let mut loops = Vec::with_capacity(vk.len());
        for i in 0..vk.len() {
            let vk0 = vk[i];
            let vk1 = vk[if i < (vk.len() - 1) { i + 1 } else { 0 }];
            if let Some(ek) = self.add_edge(vk0, vk1) {
                loops.push(Loop { vert: vk0, edge: ek });
            } else {
                // Error: failed to create edge
                return None;
            }
        }
        if let Some(val) = self.face_range_set.take_any_one() {
            let fk = FKey::new(val);
            self.faces.insert(fk, Face::new(loops));
            Some(fk)
        } else {
            None
        }
    }
}

pub struct Vert {
    edges: Vec<EKey>
}

impl Vert {
    fn new() -> Vert {
        Vert { edges: Vec::new() }
    }

    /// Check if the edge is in the set of edges adjacent to this vert.
    pub fn is_edge_adjacent(&self, ek: EKey) -> bool {
        self.edges.contains(&ek)
    }

    /// Add edge to set of edges adjacent to the vert. Does nothing if
    /// the edge is already in the set.
    fn push_edge(&mut self, ek: EKey) {
        if !self.is_edge_adjacent(ek) {
            self.edges.push(ek);
        }
    }
}

pub struct Edge {
    verts: [VKey; 2],
    // faces: Vec<FKey>
}

impl Edge {
    fn is_vert_adjacent(&self, vk: VKey) -> bool {
        self.verts.contains(&vk)
    }
}

pub struct Loop {
    vert: VKey,
    edge: EKey,
}

pub struct Face {
    loops: Vec<Loop>
}

impl Face {
    fn new(loops: Vec<Loop>) -> Face {
        Face { loops: loops }
    }
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

        // Add a valid edge
        let e = mesh.add_edge(a, b).unwrap();
        assert!(mesh.verts[&a].edges.contains(&e));
        assert!(mesh.verts[&b].edges.contains(&e));

        // Edge from a vertex to the same vertex is not allowed
        assert!(mesh.add_edge(a, a).is_none());

        // Duplicate edge should return the existing edge
        assert_eq!(mesh.add_edge(a, b).unwrap(), e);
        assert_eq!(mesh.add_edge(b, a).unwrap(), e);
    }

    #[test]
    fn test_add_face() {
        let mut mesh = Mesh::new();
        let a = mesh.add_vert().unwrap();
        let b = mesh.add_vert().unwrap();
        let c = mesh.add_vert().unwrap();

        // Add a valid triangle
        let f = mesh.add_face(&[a, b, c]).unwrap();
        // TODO(nicholasbishop): check adj

        // Add an invalid triangle with invalid edge
        assert!(mesh.add_face(&[a, b, b]).is_none());
    }
}
