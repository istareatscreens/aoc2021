use log::debug;
use priority_queue::DoublePriorityQueue;
use std::fs;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");

    let lines = data.lines().map(|s| s.to_string());

    let graph: Vec<Vec<_>> = lines
        .map(|a| {
            a.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<usize>>>();

    debug!("Chain Data: {:?}", graph);

    let mut shortest_path = Dijkstras {
        graph: graph.clone(),
        visited: vec![vec![false; graph[0].len()]; graph.len()],
        dist: vec![vec![usize::MAX; graph[0].len()]; graph.len()],
        prev: vec![vec![(0, 0); graph[0].len()]; graph.len()],
    };

    shortest_path.find_shortest_path((0, 0), (graph[0].len() - 1, graph.len() - 1));

    println!(
        "P1 Solution: {}",
        shortest_path.get_dist_of_node_from_root((graph[0].len() - 1, graph.len() - 1))
    );

    let mut tiles: Vec<Vec<Vec<usize>>> = Vec::new();

    let mut temp: Vec<Vec<usize>> = graph.clone();
    tiles.push(temp.clone());
    for _ in 0..4 {
        temp = create_2d_tile(&temp.to_owned());
        tiles.push(temp.clone());
    }

    let mut final_graph: Vec<Vec<usize>> = vec![Vec::new(); graph.len()];
    for tile in tiles.clone().iter() {
        for i in 0..tile.len() {
            final_graph[i].append(&mut tile[i].clone());
        }
    }
    debug!("right expand final graph {:?}", final_graph);

    let mut final_tiles: Vec<Vec<Vec<usize>>> = Vec::new();
    temp = final_graph.clone();
    for _ in 0..4 {
        temp = create_2d_tile(&temp.to_owned());
        final_tiles.push(temp.clone());
    }

    debug!("final tiles {:?}", final_tiles);
    for tile in final_tiles.iter() {
        for row in tile.iter() {
            final_graph.push(row.clone());
        }
    }

    debug!("final graph {:?}", final_graph);

    shortest_path = Dijkstras {
        graph: final_graph.clone(),
        visited: vec![vec![false; final_graph[0].len()]; final_graph.len()],
        dist: vec![vec![usize::MAX; final_graph[0].len()]; final_graph.len()],
        prev: vec![vec![(0, 0); final_graph[0].len()]; final_graph.len()],
    };

    shortest_path.find_shortest_path((0, 0), (final_graph[0].len() - 1, final_graph.len() - 1));

    println!(
        "P2 Solution: {}",
        shortest_path.get_dist_of_node_from_root((final_graph[0].len() - 1, final_graph.len() - 1))
    );
}

fn create_2d_tile(array: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut new_array: Vec<Vec<usize>> = Vec::new();
    for item in array.iter() {
        new_array.push(tile_array(item));
    }
    new_array
}

fn tile_array(array: &[usize]) -> Vec<usize> {
    let mut new_array: Vec<usize> = Vec::new();
    for element in array.iter() {
        let new_value = element + 1;
        new_array.push(if new_value > 9 { 1 } else { new_value });
    }
    new_array
}

struct Dijkstras {
    graph: Vec<Vec<usize>>,
    visited: Vec<Vec<bool>>,
    dist: Vec<Vec<usize>>,
    prev: Vec<Vec<(usize, usize)>>,
}

impl Dijkstras {
    pub fn get_dist_of_node_from_root(&self, end_index: (usize, usize)) -> usize {
        self.dist[end_index.1][end_index.0]
    }

    fn path_find(&mut self, root_index: (usize, usize)) {
        let mut queue: DoublePriorityQueue<(usize, usize), usize> = DoublePriorityQueue::new();
        self.dist[root_index.1][root_index.0] = 0;
        queue.push(root_index, 0);
        while !queue.is_empty() {
            let result = queue.pop_min().unwrap();
            self.visited[result.0 .1][result.0 .0] = true;
            if self.dist[result.0 .1][result.0 .0] < result.1 {
                continue;
            }
            let neighbours: Vec<(usize, usize)> =
                self.get_neighbours(result.0 .1, result.0 .0, false);
            for neighbour in neighbours.iter() {
                if self.visited[neighbour.1][neighbour.0] {
                    continue;
                }
                let new_dist =
                    self.dist[result.0 .1][result.0 .0] + self.graph[neighbour.1][neighbour.0];
                if new_dist < self.dist[neighbour.1][neighbour.0] {
                    self.prev[neighbour.1][neighbour.0] = (result.0 .0, result.0 .1);
                    self.dist[neighbour.1][neighbour.0] = new_dist;
                    queue.push((neighbour.0, neighbour.1), new_dist);
                }
            }
        }
    }

    pub fn find_shortest_path(
        &mut self,
        root_index: (usize, usize),
        end_index: (usize, usize),
    ) -> Vec<(usize, usize)> {
        self.path_find(root_index);
        let mut shortest_path: Vec<(usize, usize)> = Vec::new();
        let mut i = end_index.1;
        let mut j = end_index.0;
        while i != root_index.1 && j != root_index.0 {
            let (new_j, new_i) = self.prev[i][j];
            i = new_i;
            j = new_j;
            shortest_path.push((j, i));
        }
        shortest_path
    }

    fn get_neighbours(&self, i: usize, j: usize, diagonals: bool) -> Vec<(usize, usize)> {
        let mut neighbours: Vec<(usize, usize)> = Vec::new();

        let check = |i: i32, j: i32| {
            self.check_if_index_exists(&self.graph, &i)
                && self.check_if_index_exists(&self.graph[0], &j)
        };

        if self.check_if_index_exists(&self.graph[i], &(j as i32 + 1)) {
            neighbours.push((j + 1, i));
        }
        if self.check_if_index_exists(&self.graph[i], &(j as i32 - 1)) {
            neighbours.push((j - 1, i));
        }
        if self.check_if_index_exists(&self.graph, &(i as i32 + 1)) {
            neighbours.push((j, i + 1));
        }
        if self.check_if_index_exists(&self.graph, &(i as i32 - 1)) {
            neighbours.push((j, i - 1));
        }
        if diagonals && check(i as i32 + 1, j as i32 + 1) {
            neighbours.push((j + 1, i + 1));
        }
        if diagonals && check(i as i32 - 1, j as i32 - 1) {
            neighbours.push((j - 1, i - 1));
        }
        if diagonals && check(i as i32 + 1, j as i32 - 1) {
            neighbours.push((j - 1, i + 1));
        }
        if diagonals && check(i as i32 - 1, j as i32 + 1) {
            neighbours.push((j + 1, i - 1));
        }

        neighbours
    }

    fn check_if_index_exists<T>(&self, vec: &[T], index: &i32) -> bool {
        index < &(vec.len() as i32) && index > &-1
    }
}
