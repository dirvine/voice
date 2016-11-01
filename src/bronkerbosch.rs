use std::collections::HashSet;
use petgraph::graphmap::{NodeTrait, UnGraphMap};

/// Implementation of "Algorithm 457: Finding All Cliques of an Undirected Graph"
/// by Bronand Kerbosch; http://doi.acm.org/10.1145/362342.362367
pub struct BronKerbosch<N: NodeTrait, E> {
    graph: UnGraphMap<N, E>,
    cliques: Vec<HashSet<N>>,
}

impl<N: NodeTrait, E> BronKerbosch<N, E> {
    ///
    pub fn new(graphmap: UnGraphMap<N, E>) -> BronKerbosch<N, E> {
        BronKerbosch {
            graph: graphmap,
            cliques: Vec::new(),
        }
    }
    ///
    pub fn compute(&mut self) {
        let mut p = self.graph.nodes().collect::<HashSet<N>>();
        let mut r = HashSet::new();
        let mut x = HashSet::new();
        self.bronkerbosch(&mut p, &mut r, &mut x);
    }
    ///
    pub fn cliques(&self) -> &Vec<HashSet<N>> {
        &self.cliques
    }


    fn bronkerbosch(&mut self, p: &mut HashSet<N>, r: &mut HashSet<N>, x: &mut HashSet<N>) {

        if p.is_empty() && x.is_empty() {
            self.cliques.push(r.clone());
        }

        let clone = p.iter().cloned().collect::<HashSet<N>>();

        for v in clone.iter() {
            let v_neighbours = self.graph.neighbors(v.clone()).collect::<HashSet<N>>();
            let mut p_intersect_v_neighbors = p.intersection(&v_neighbours).cloned().collect();
            r.insert(v.clone());
            let mut x_intersect_v_neighbors = x.intersection(&v_neighbours).cloned().collect();

            self.bronkerbosch(&mut p_intersect_v_neighbors,
                              r,
                              &mut x_intersect_v_neighbors);

            p.remove(v);
            x.insert(*v);
        }

    }
}
