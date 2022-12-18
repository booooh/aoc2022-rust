use std::{
    collections::{HashMap, VecDeque},
    fs,
    hash::Hash,
    str::FromStr,
    string::ParseError,
};

#[derive(Debug)]
struct DirListing {
    name: String,
    dir_entry: DirEntry,
}

#[derive(Debug)]
enum DirEntry {
    FILE(usize),
    DIRECTORY,
}

impl FromStr for DirListing {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("dir") {
            return Ok(DirListing {
                name: s.split(" ").nth(1).unwrap().into(),
                dir_entry: DirEntry::DIRECTORY,
            });
        } else {
            let mut split_line = s.split(" ");
            let file_size = split_line.next().unwrap();
            let file_name = split_line.next().unwrap();
            return Ok(DirListing {
                name: file_name.into(),
                dir_entry: DirEntry::FILE(file_size.parse::<usize>().unwrap()),
            });
        }
    }
}

fn concat_path(dirname: &str, basename: &str) -> String {
    format!("{}/{}", dirname, basename).replace("//", "/")
}

impl FromStr for Arena<String, DirListing> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let mut arena = Arena::<String, _>::new();

        // add the root node
        arena.add_node(
            "/".into(),
            DirListing {
                name: "/".into(),
                dir_entry: DirEntry::DIRECTORY,
            },
            None,
        );
        let mut curr_dir_name: String = "".into();
        while let Some(line) = lines.next() {
            // we always reach here with the next line being a command
            if line.starts_with("$ cd") {
                let name = line.split(" ").nth(2).unwrap();
                // let tmp_curr_dir_name = &curr_dir_name;
                if name == ".." {
                    curr_dir_name = arena
                        .nodes
                        .get(&curr_dir_name)
                        .unwrap()
                        .parent
                        .clone()
                        .unwrap();
                } else {
                    curr_dir_name = concat_path(&curr_dir_name, name);
                }
            } else {
                // $ ls
                while let Some(next_line) = lines.peek() {
                    // break from the loop if the next line is a command
                    if next_line.starts_with("$") {
                        break;
                    }

                    let dir_listing: DirListing = lines.next().unwrap().parse().unwrap();
                    arena.add_node(
                        concat_path(&curr_dir_name, &dir_listing.name),
                        dir_listing,
                        Some(curr_dir_name.clone()),
                    );
                }
            }
        }

        Ok(arena)
    }
}

#[derive(Debug)]
struct TreeNode<ID, T>
where
    ID: Hash + Eq + PartialEq + Clone,
{
    id: ID,
    data: T,
    parent: Option<ID>,
    children: Vec<ID>,
}

impl<ID, T> TreeNode<ID, T>
where
    ID: Hash + Eq + PartialEq + Clone,
{
    fn new(id: ID, data: T, parent: Option<ID>) -> Self {
        Self {
            id,
            data,
            parent,
            children: vec![],
        }
    }

    fn add_child(&mut self, child_id: ID) {
        self.children.push(child_id);
    }

    fn remove_child(&mut self, child_id: ID) {
        self.children
            .remove(self.children.iter().position(|x| x == &child_id).unwrap());
    }
}

impl Arena<String, DirListing> {
    fn size(&self, node_id: &String, size_memo: &mut HashMap<String, usize>) -> usize {
        let node = self.nodes.get(node_id).unwrap();
        if let Some(size_val) = size_memo.get(node_id) {
            return *size_val;
        }

        let size = match node.data.dir_entry {
            DirEntry::FILE(file_size) => file_size,
            DirEntry::DIRECTORY => {
                let dir_size = node
                    .children
                    .iter()
                    .map(|child| self.size(child, size_memo))
                    .sum();
                size_memo.insert(node_id.into(), dir_size);
                dir_size
            }
        };
        return size;
    }
}

struct Arena<ID, T>
where
    ID: Hash + Eq + PartialEq + Clone,
{
    nodes: HashMap<ID, TreeNode<ID, T>>,
}

struct DfsIterator<'a, ID, T>
where
    ID: Hash + Eq + PartialEq + Clone,
{
    arena: &'a Arena<ID, T>,
    stack: VecDeque<ID>,
}

impl<'a, ID, T> Iterator for DfsIterator<'a, ID, T>
where
    ID: Hash + Eq + PartialEq + Clone,
{
    type Item = ID;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr_index) = self.stack.pop_front() {
            // get the children of the current node, and push them so the left-most child is added last, to be popped-off first
            let children = &self.arena.nodes.get(&curr_index).unwrap().children;
            for child in children.iter().rev() {
                self.stack.push_front(child.clone())
            }

            return Some(curr_index);
        }
        return None;
    }
}

impl<ID, T> Arena<ID, T>
where
    ID: Hash + Eq + PartialEq + Clone + std::fmt::Debug,
{
    fn add_node(&mut self, key: ID, data: T, parent_key: Option<ID>) -> &TreeNode<ID, T> {
        // add the element to the arena
        self.nodes.insert(
            key.clone(),
            TreeNode::new(key.clone(), data, parent_key.clone()),
        );

        // properly associate it to the parent
        if let Some(parent_key_val) = parent_key {
            let parent_node = self.nodes.get_mut(&parent_key_val).unwrap();
            parent_node.add_child(key.clone());
        }

        return self.nodes.get(&key).unwrap();
    }

    fn remove_node(&mut self, key: ID) -> Option<TreeNode<ID, T>> {
        // if the node to remove has a parent, remove this node from the parent's children
        if let Some(parent_id) = self.nodes.get(&key).unwrap().parent.clone() {
            self.nodes
                .get_mut(&parent_id)
                .unwrap()
                .remove_child(key.clone());
        }

        // remove the node
        self.nodes.remove(&key)
    }

    fn get(&self, key: &ID) -> Option<&TreeNode<ID, T>> {
        return self.nodes.get(key);
    }

    fn get_mut(&mut self, key: &ID) -> Option<&mut TreeNode<ID, T>> {
        return self.nodes.get_mut(key);
    }

    fn new() -> Self {
        return Self {
            nodes: HashMap::new(),
        };
    }

    fn dfs(&self, start_node: ID) -> DfsIterator<ID, T> {
        //doing a pre-order walk of the tree, so we yield each node as we visit it, starting with the start_node
        DfsIterator {
            arena: self,
            stack: vec![start_node].into(),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    source_stack: usize,
    dest_stack: usize,
    count: usize,
}

fn main() {
    let mut tree = Arena::<usize, String>::new();
    tree.add_node(1, "foo".to_owned(), None);
    tree.add_node(2, "bar".to_owned(), Some(1));
    tree.add_node(82, "baz".to_owned(), Some(1));
    tree.add_node(42, "bam".to_owned(), Some(2));
    tree.add_node(92, "boom".to_owned(), Some(1));

    // tree is expected to be:
    //          1
    //       2     82   92
    //      42

    for node_key in tree.dfs(1) {
        println!("{}", node_key);
    }
    let contents = fs::read_to_string("input/day7.txt").unwrap();
    let mut tree_new: Arena<String, DirListing> = contents.parse().unwrap();
    for node_key in tree_new.dfs("/".into()) {
        println!("{}", node_key);
    }

    let mut size_memo = HashMap::new();
    tree_new.size(&"/".into(), &mut size_memo);
    let total_used = size_memo.get("/").unwrap();
    let total_free = 70000000 - total_used;
    let needed = 30000000 - total_free;
    dbg!(total_free, &needed);

    //dbg!(&size_memo);
    let mut res = size_memo
        .iter()
        .map(|(_, size)| size)
        .filter(|&size| size >= &needed)
        .collect::<Vec<_>>();
    res.sort();
    dbg!(res);
}
