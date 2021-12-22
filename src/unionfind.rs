#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use cargo_snippet::snippet;

#[snippet("UnionFind-struct")]
struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}
#[snippet("UnionFind")]
#[snippet(include = "UnionFind-struct")]
impl UnionFind {
    fn new(len: usize) -> UnionFind {
        return UnionFind {
            parents: (0..len).map(|i| i).collect(),
            ranks: vec![0; len],
        };
    }
    fn root(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            let root_of_parent = self.root(self.parents[x]);
            self.parents[x] = root_of_parent;
            root_of_parent
        }
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        return self.root(x) == self.root(y);
    }
    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.root(x);
        let root_y = self.root(y);
        if root_x == root_y {
            return;
        }
        if self.ranks[root_x] < self.ranks[root_y] {
            self.parents[root_x] = root_y;
        } else {
            self.parents[root_y] = root_x;
            if self.ranks[root_x] == self.ranks[root_y] {
                self.ranks[root_x] += 1;
            }
        }
    }
}

#[test]
fn test_union_find() {
    let mut uf = UnionFind::new(5);
    uf.union(1, 2);
    uf.union(3, 2);
    assert_eq!(uf.same(1,3), true);
    assert_eq!(uf.same(1,4), false);
    uf.union(2,4);
    assert_eq!(uf.same(4,1), true);
    uf.union(4,2);
    uf.union(0,0);
    assert_eq!(uf.same(0,0), true);
}
