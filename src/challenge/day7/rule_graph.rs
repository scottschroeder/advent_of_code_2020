use petgraph::graphmap::DiGraphMap;
use std::{collections::HashMap, fmt};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Bag<'a>(pub &'a str);

impl<'a> fmt::Debug for Bag<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

type RuleGraph<'a> = DiGraphMap<Bag<'a>, usize>;
#[derive(Debug, Default)]
pub struct Rules<'a> {
    g: RuleGraph<'a>,
}

impl<'a> Rules<'a> {
    pub fn insert(&mut self, src: Bag<'a>, dst: Bag<'a>, count: usize) {
        self.g.add_edge(src, dst, count);
    }
    pub fn terminal(&mut self, bag: Bag<'a>) {
        self.g.add_node(bag);
    }
    pub fn parents_of(&self, bag: &str) -> usize {
        let bag = Bag(bag);
        let g = petgraph::visit::Reversed(&self.g);
        let mut dfs = petgraph::visit::Dfs::new(&g, bag);
        let mut c = 0;
        while let Some(x) = dfs.next(&g) {
            if x == bag {
                continue;
            }
            log::trace!("{:?}", x);

            c += 1
        }
        c
    }
    pub fn total_bags(&self, bag: &str) -> usize {
        let mut cache = HashMap::default();
        let bag = Bag(bag);
        recursive_bags(&self.g, bag, &mut cache)
    }
}

fn recursive_bags<'a>(
    g: &RuleGraph<'a>,
    bag: Bag<'a>,
    cache: &mut HashMap<Bag<'a>, usize>,
) -> usize {
    if let Some(s) = cache.get(&bag) {
        return *s;
    }
    let mut total = 0usize;
    for (_, target, amt) in g.edges(bag) {
        let inner = recursive_bags(g, target, cache);
        total += amt * (inner + 1);
    }
    cache.insert(bag, total);
    total
}
