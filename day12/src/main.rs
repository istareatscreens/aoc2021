use log::debug;
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    //let data = fs::read_to_string("input.test.txt").expect("Unable to read file");

    let connections: Vec<Vec<_>> = data
        .lines()
        .map(|a: &str| {
            a.to_string()
                .split('-')
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    debug!("Input Data: {:?}", connections);

    let mut adjacency_list: HashMap<String, Vec<String>> = HashMap::new();
    let mut root_nodes: Vec<String> = Vec::new();
    for connection in connections {
        if connection[1] == *"start".to_string() {
            root_nodes.push(connection[0].to_owned());
            continue;
        }
        if connection[0] == *"start".to_string() {
            root_nodes.push(connection[1].to_owned());
            continue;
        }

        if !adjacency_list.contains_key(&connection[0]) {
            adjacency_list.insert(connection[0].to_owned(), vec![connection[1].to_owned()]);
        } else {
            adjacency_list
                .get_mut(&connection[0])
                .unwrap()
                .push(connection[1].to_owned());
        }
        if !adjacency_list.contains_key(&connection[1]) {
            adjacency_list.insert(connection[1].to_owned(), vec![connection[0].to_owned()]);
        } else {
            adjacency_list
                .get_mut(&connection[1])
                .unwrap()
                .push(connection[0].to_owned());
        }
    }
    adjacency_list.remove_entry("end");

    debug!("Roots: {:?}", root_nodes);
    for (key, path) in adjacency_list.iter() {
        debug!("Key: {} Adjacency Paths: {:?}", key, path);
    }
    debug!("");

    let mut path_search = PathSearch {
        path_list: HashMap::new(),
        successful_paths: HashMap::new(),
        adjacency_list: adjacency_list.clone(),
    };

    for root in root_nodes.iter() {
        path_search.find_all_paths(root.to_string());
    }

    let successful_paths = path_search.get_successful_paths();

    for (key, path) in successful_paths.iter() {
        debug!("Key: {} Winning Paths: {:?}", key, path);
    }
    debug!("");

    for (key, path) in path_search.get_path_list().iter() {
        debug!("All: {} visited?: {}", key, path);
    }
    debug!("");

    println!("P1 Solution: {}", successful_paths.len());

    debug!("Roots: {:?}", root_nodes);
    for (key, path) in adjacency_list.iter() {
        debug!("Key: {} Adjacency Paths: {:?}", key, path);
    }
    debug!("");

    let mut path_search2 = PathSearch {
        path_list: HashMap::new(),
        successful_paths: HashMap::new(),
        adjacency_list: adjacency_list.clone(),
    };

    for root in root_nodes.iter() {
        path_search2.find_all_paths_with_extra_rule(root.to_string());
    }

    for (key, path) in path_search2.get_successful_paths().iter() {
        debug!("Key: {} Winning Paths: {:?}", key, path);
    }
    debug!("");

    for (key, path) in path_search2.get_path_list().iter() {
        debug!("All: {} visited?: {}", key, path);
    }
    debug!("");

    println!("P2 Solution: {}", path_search2.get_successful_paths().len());
}

struct PathSearch {
    path_list: HashMap<String, bool>,
    successful_paths: HashMap<String, Vec<String>>,
    adjacency_list: HashMap<String, Vec<String>>,
}

impl PathSearch {
    pub fn get_successful_paths(&self) -> HashMap<String, Vec<String>> {
        self.successful_paths.clone()
    }

    pub fn get_path_list(&self) -> HashMap<String, bool> {
        self.path_list.clone()
    }

    pub fn find_all_paths(&mut self, root: String) {
        let mut queue: VecDeque<Vec<String>> = VecDeque::new();
        queue.push_back(vec![root]);

        while !queue.is_empty() {
            let path = queue.pop_front().unwrap();
            let last_node = &path[&path.len() - 1];

            let key: String = self.make_key(&path);
            self.path_list.insert(key.to_owned(), false);
            let successful_path_already_found = self.successful_paths.contains_key(&key.to_owned());
            if last_node == "end" && !successful_path_already_found {
                self.successful_paths.insert(key.to_owned(), path);
            } else if successful_path_already_found {
                continue;
            } else {
                let neighbours: Vec<String> = self.adjacency_list.get(last_node).unwrap().to_vec();
                for neighbour in neighbours.iter() {
                    let mut new_path: Vec<String> = path.clone();

                    if self.check_if_already_visited(&new_path, neighbour.to_string()) {
                        continue;
                    }

                    new_path.push(neighbour.to_string());
                    let key = self.make_key(&new_path);

                    if self.path_list.contains_key(&key.to_owned())
                        && *self.path_list.get(&key.to_owned()).unwrap()
                    {
                        continue;
                    }
                    queue.push_back(new_path);
                }
            }
            *self.path_list.get_mut(&key.to_owned()).unwrap() = true;
        }
    }

    pub fn find_all_paths_with_extra_rule(&mut self, root: String) {
        let mut queue: VecDeque<Vec<String>> = VecDeque::new();
        queue.push_back(vec![root]);

        while !queue.is_empty() {
            let path = queue.pop_front().unwrap();
            let last_node = &path[&path.len() - 1];

            let key: String = self.make_key(&path);
            self.path_list.insert(key.to_owned(), false);
            let successful_path_already_found = self.successful_paths.contains_key(&key.to_owned());
            if last_node == "end" && !successful_path_already_found {
                self.successful_paths.insert(key.to_owned(), path);
            } else if successful_path_already_found {
                continue;
            } else {
                let neighbours: Vec<String> = self.adjacency_list.get(last_node).unwrap().to_vec();
                for neighbour in neighbours.iter() {
                    let mut new_path: Vec<String> = path.clone();

                    if self.check_if_already_visited_twice(&new_path, neighbour.to_string()) {
                        continue;
                    }

                    new_path.push(neighbour.to_string());
                    let key = self.make_key(&new_path);

                    if self.path_list.contains_key(&key.to_owned())
                        && *self.path_list.get(&key.to_owned()).unwrap()
                    {
                        continue;
                    }
                    queue.push_back(new_path);
                }
            }
            *self.path_list.get_mut(&key.to_owned()).unwrap() = true;
        }
    }

    fn check_if_already_visited(&self, path: &[String], node: String) -> bool {
        let check_if_uppercase =
            |node: &String, index: usize| node.chars().nth(index).unwrap().is_ascii_uppercase();
        !(node.len() == 2 && check_if_uppercase(&node, 0)) && path.contains(&node)
    }

    fn check_if_already_visited_twice(&self, path: &[String], node: String) -> bool {
        let check_if_lower_case = |node: &String, index: usize| {
            node.len() == 2 && node.chars().nth(index).unwrap().is_ascii_lowercase()
        };

        let mut contains_two_lower = false;
        for n in path {
            let mut count = 0;
            for n_compare in path {
                if check_if_lower_case(n, 0)
                    && check_if_lower_case(n_compare, 0)
                    && *n_compare == *n
                {
                    count += 1;
                }
                if count == 2 {
                    contains_two_lower = true;
                    break;
                }
            }
        }

        let check_if_uppercase =
            |node: &String, index: usize| node.chars().nth(index).unwrap().is_ascii_uppercase();
        !(node.len() == 2 && check_if_uppercase(&node, 0))
            && (contains_two_lower && path.contains(&node))
    }

    fn make_key(&self, strings: &[String]) -> String {
        return strings
            .iter()
            .map(|s| s.to_owned() + ",")
            .collect::<Vec<String>>()
            .concat();
    }
}
