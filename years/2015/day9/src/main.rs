use std::cell::RefCell;
use std::collections::HashMap;

/* ---------- */

type Routes = HashMap<&'static str, u32>;
type Cities = HashMap<&'static str, Routes>;

/* ---------- */

fn new_path(graph: &mut Cities, from: &'static str, to: &'static str, dist: u32) {
    match graph.get_mut(from) {
        Some(routes) => {
            match routes.get_mut(to) {
                Some(_) => panic!("distance already set"),
                None => { routes.insert(to, dist); }
            }
        },
        None => {
            let mut new_route = Routes::default();

            new_route.insert(to, dist);
            graph.insert(from, new_route);
        }
    }
}

/* ---------- */

fn visit(graph: &Cities, visited: &RefCell<Vec<&'static str>>, min_distance: &mut u32) {
    graph.iter()
        .filter(|(name, _)| {
            !visited.borrow().iter().any(|visited| **name == *visited)
        }).for_each(|(city, _routes)| {
            visited.borrow_mut().push(city);
            visit(graph, visited, min_distance);
            visited.borrow_mut().pop();
        });

    if graph.len() == visited.borrow().len() {
        let distance_visited = total_distance(graph, visited);

        if distance_visited < *min_distance {
            *min_distance = distance_visited;
        }
    }
}

/* ---------- */

fn total_distance(graph: &Cities, visited: &RefCell<Vec<&'static str>>) -> u32 {
    let visited = &*visited.borrow();
    let len = visited.len();
    let mut total_dist = 0u32;

    for idx in 0..len - 1 {
        total_dist += get_distance_to(graph, visited[idx], visited[idx + 1]);
    }

    total_dist
}

/* ---------- */

fn get_distance_to(graph: &Cities, from: &'static str, to: &'static str) -> u32 {
    if let Some(routes) = graph.get(from) {
        if let Some(dist) = routes.get(to) {
            return *dist;
        }
    }

    0
}

/* ---------- */

fn main() {
    let mut graph = Cities::default();

    utils::input_str!().lines()
        .for_each(|line | {
            let parts = line.split_whitespace().collect::<Vec<&str>>();

            let city1 = parts[0];
            let city2 = parts[2];
            let dist = parts[4].parse().expect("failed to parse distance");

            new_path(&mut graph, city1, city2, dist);
            new_path(&mut graph, city2, city1, dist)
        });

    let visited: RefCell<Vec<&'static str>> = RefCell::default();
    let mut min_dist = u32::MAX;

    visit(&graph, &visited, &mut min_dist);

    println!("result => {}", min_dist)
}
