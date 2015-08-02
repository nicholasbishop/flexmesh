mod key;
mod rangeset;

use key::Key;
use rangeset::{Range, RangeSet};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct VKeyMarker;
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct EKeyMarker;
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct FKeyMarker;
pub type VKey = Key<VKeyMarker>;
pub type EKey = Key<EKeyMarker>;
pub type FKey = Key<FKeyMarker>;
pub type FaceLen = u32;

pub struct Mesh<VData, EData, FData> {
    verts: HashMap<VKey, Vert<VData>>,
    edges: HashMap<EKey, Edge<EData>>,
    faces: HashMap<FKey, Face<FData>>,

    vert_range_set: RangeSet,
    edge_range_set: RangeSet,
    face_range_set: RangeSet
}

// TODO
fn new_key_range_set() -> RangeSet {
    RangeSet::new(Range::new(0, 0xffffffff - 1))
}

impl<VData: Clone, EData: Clone, FData: Clone> Mesh<VData, EData, FData> {
    pub fn new() -> Mesh<VData, EData, FData> {
        Mesh {
            verts: HashMap::new(),
            edges: HashMap::new(),
            faces: HashMap::new(),

            vert_range_set: new_key_range_set(),
            edge_range_set: new_key_range_set(),
            face_range_set: new_key_range_set()
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
    pub fn add_vert(&mut self, vdata: VData) -> Option<VKey> {
        if let Some(val) = self.vert_range_set.take_any_one() {
            let vkey = VKey::new(val);
            self.verts.insert(vkey, Vert::new(vdata));
            Some(vkey)
        } else {
            None
        }
    }

    pub fn add_edge(&mut self, vk0: VKey, vk1: VKey, edata: EData) -> Option<EKey> {
        // TODO(nicholasbishop): check that the verts are in the mesh,
        if vk0 == vk1 {
            None
        } else if let Some(ek) = self.find_edge(vk0, vk1) {
            Some(ek)
        } else {
            if let Some(val) = self.edge_range_set.take_any_one() {
                let ekey = EKey::new(val);
                self.verts.get_mut(&vk0).unwrap().push_edge(ekey);
                self.verts.get_mut(&vk1).unwrap().push_edge(ekey);
                self.edges.insert(ekey, Edge::new(vk0, vk1, edata));
                Some(ekey)
            } else {
                None
            }
        }
    }

    pub fn add_face(&mut self, vk: &[VKey], edata: EData, fdata: FData) -> Option<FKey> {
        let mut loops = Vec::with_capacity(vk.len());
        for i in 0..vk.len() {
            let vk0 = vk[i];
            let vk1 = vk[if i < (vk.len() - 1) { i + 1 } else { 0 }];
            // TODO(nicholasbishop): do more checking up front so that
            // edges don't get created on failure, or delete them after.
            if let Some(ek) = self.add_edge(vk0, vk1, edata.clone()) {
                loops.push(Loop { vert: vk0, edge: ek });
            } else {
                // Error: failed to create edge
                return None;
            }
        }
        if let Some(val) = self.face_range_set.take_any_one() {
            let fk = FKey::new(val);
            for lp in loops.iter() {
                self.edges.get_mut(&lp.edge).unwrap().push_face(fk);
            }
            self.faces.insert(fk, Face::new(loops, fdata));
            Some(fk)
        } else {
            None
        }
    }
}

pub struct Vert<VData> {
    vdata: VData,
    edges: Vec<EKey>
}

impl<VData> Vert<VData> {
    fn new(vdata: VData) -> Vert<VData> {
        Vert { vdata: vdata, edges: Vec::new() }
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

pub struct Edge<EData> {
    edata: EData,
    verts: [VKey; 2],
    faces: Vec<FKey>
}

impl<EData> Edge<EData> {
    fn new(vk0: VKey, vk1: VKey, edata: EData) -> Edge<EData> {
        Edge { edata: edata, verts: [vk0, vk1], faces: Vec::new() }
    }

    /// Check if the vert is one of the two verts adjacent to this edge.
    fn is_vert_adjacent(&self, vk: VKey) -> bool {
        self.verts.contains(&vk)
    }

    /// Check if the face is in the set of edges adjacent to this edge.
    pub fn is_face_adjacent(&self, fk: FKey) -> bool {
        self.faces.contains(&fk)
    }

    /// Add face to set of faces adjacent to the edge. Does nothing if
    /// the face is already in the set.
    fn push_face(&mut self, fk: FKey) {
        if !self.is_face_adjacent(fk) {
            self.faces.push(fk);
        }
    }
}

pub struct Loop {
    vert: VKey,
    edge: EKey,
}

pub struct Face<FData> {
    fdata: FData,
    loops: Vec<Loop>
}

impl<FData> Face<FData> {
    fn new(loops: Vec<Loop>, fdata: FData) -> Face<FData> {
        Face { fdata: fdata, loops: loops }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Placeholder for vert/edge/face data
    const DAT: () = ();

    #[test]
    fn test_add_vert() {
        let mut mesh = Mesh::<_, (), ()>::new();
        assert!(mesh.add_vert(DAT).is_some());
        assert!(!mesh.verts.is_empty());
    }

    #[test]
    fn test_add_edge() {
        let mut mesh = Mesh::<_, _, ()>::new();
        let a = mesh.add_vert(DAT).unwrap();
        let b = mesh.add_vert(DAT).unwrap();

        // Add a valid edge
        let e = mesh.add_edge(a, b, DAT).unwrap();
        assert!(mesh.verts[&a].edges.contains(&e));
        assert!(mesh.verts[&b].edges.contains(&e));

        // Edge from a vertex to the same vertex is not allowed
        assert!(mesh.add_edge(a, a, DAT).is_none());

        // Duplicate edge should return the existing edge
        assert_eq!(mesh.add_edge(a, b, DAT).unwrap(), e);
        assert_eq!(mesh.add_edge(b, a, DAT).unwrap(), e);
    }

    #[test]
    fn test_add_face() {
        let mut mesh = Mesh::new();
        let a = mesh.add_vert(DAT).unwrap();
        let b = mesh.add_vert(DAT).unwrap();
        let c = mesh.add_vert(DAT).unwrap();

        // Add a valid triangle
        let fk = mesh.add_face(&[a, b, c], DAT, DAT).unwrap();
        assert_eq!(mesh.edges.len(), 3);
        for lp in mesh.faces[&fk].loops.iter() {
            assert!(mesh.edges[&lp.edge].is_face_adjacent(fk));
        }

        // Add an invalid triangle with invalid edge
        assert!(mesh.add_face(&[a, b, b], DAT, DAT).is_none());
    }
}
