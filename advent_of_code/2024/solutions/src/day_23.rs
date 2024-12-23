// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2024-day23/

use crate::{AoCData, AoCResult};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug, Clone)]
pub struct Data<'a>(HashMap<&'a str, HashSet<&'a str>>);

fn bron_kerbosch<'a>(
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    req: &mut HashSet<&'a str>,
    mut keys: HashSet<&'a str>,
    mut todo: HashSet<&'a str>,
    cliques: &mut Vec<HashSet<&'a str>>,
) {
    if keys.is_empty() {
        if todo.is_empty() {
            cliques.push(req.clone());
        }
        return;
    }
    while let Some(neighbour) = keys.iter().copied().next() {
        let neighbours = &graph[neighbour];
        let new_keys = keys
            .intersection(neighbours)
            .copied()
            .collect();
        let new_todo = todo
            .intersection(neighbours)
            .copied()
            .collect();

        req.insert(neighbour);
        bron_kerbosch(graph, req, new_keys, new_todo, cliques);
        req.remove(neighbour);

        keys.remove(neighbour);
        todo.insert(neighbour);
    }
}

// fn search<'a>(
//     name: &'a str,
//     req: BTreeSet<&'a str>,
//     sets: &mut HashSet<BTreeSet<&'a str>>,
//     connections: &HashMap<&'a str, HashSet<&'a str>>,
// ) {
//     if sets.contains(&req) {
//         return;
//     }
//     sets.insert(req.clone());
//     for neighbour in &connections[name] {
//         if req.contains(neighbour) {
//             continue;
//         }
//         if !req
//             .iter()
//             .all(|query| connections[query].contains(neighbour))
//         {
//             continue;
//         }
//         let mut new_req = req.clone();
//         new_req.insert(neighbour);
//         search(neighbour, new_req, sets, connections)
//     }
// }

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
        for line in input.lines() {
            let (left, right) = line.split_once('-').unwrap();
            connections
                .entry(left)
                .or_default()
                .insert(right);
            connections
                .entry(right)
                .or_default()
                .insert(left);
        }

        Ok(Self(connections))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut sets = HashSet::new();
        for pc1 in self.0.keys() {
            // only count sets where first pc might be the chief historian's to avoid triplecounting
            if !pc1.starts_with('t') {
                continue;
            }
            // pcs connected to pc1
            for pc2 in &self.0[pc1] {
                // pcs connected to both pc1 and pc2
                for pc3 in self.0[pc1].intersection(&self.0[pc2]) {
                    let set = BTreeSet::from([pc1, pc2, pc3]);
                    sets.insert(set);
                }
            }
        }

        Ok(sets.len())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut cliques = Vec::new();
        let pcs = self.0.keys().copied().collect();

        bron_kerbosch(
            &self.0,
            &mut HashSet::new(),
            pcs,
            HashSet::new(),
            &mut cliques,
        );

        let mut clique: Vec<_> = cliques
            .into_iter()
            .max_by_key(|clique| clique.len())
            .unwrap()
            .into_iter()
            .collect();
        clique.sort();
        Ok(clique.join(","))

        // OR
        // let mut sets = HashSet::new();
        // for &name in connections.keys() {
        //     search(name, BTreeSet::from([name]), &mut sets, &connections);
        // }
        // let mut names: Vec<_> = sets
        //     .into_iter()
        //     .max_by_key(|set| set.len())
        //     .unwrap()
        //     .into_iter()
        //     .collect();
        // names.sort();
        // names.join(",")
        //
        // OR, much faster but relies on specifically structured input
        // let mut largest = HashSet::new();
        //
        // for (&name, neighbours) in &connections {
        //     let mut group = HashSet::new();
        //     group.insert(name);
        //
        //     for &neighbour in neighbours {
        //         // if neighbour is connected to all group members, it joins the group
        //         let new_neighbours = connections.get(neighbour).unwrap();
        //         if group.is_subset(new_neighbours) {
        //             group.insert(neighbour);
        //         }
        //     }
        //
        //     if group.len() > largest.len() {
        //         largest = group;
        //     }
        // }
        //
        // let mut names: Vec<_> = largest.into_iter().collect();
        // names.sort();
        // names.join(",")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "7");
    }

    #[test]
    fn part_2() {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "co,de,ka,ta");
    }
}
