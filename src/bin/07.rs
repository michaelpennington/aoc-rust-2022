use advent_of_code::util::tree::{Node, SharedNode, Tree};

advent_of_code::solution!(7);

fn fill_sizes(node: &SharedNode<FsNode<'_>>) -> u64 {
    let mut total = 0;
    for child in Node::iter_children(node) {
        let ty = child.borrow().value.ty;
        match ty {
            NodeT::File { size } | NodeT::Dir { size: Some(size) } => total += size,
            NodeT::Dir { size: None } => total += fill_sizes(&child),
        }
    }
    if let NodeT::Dir { ref mut size } = node.borrow_mut().value.ty {
        *size = Some(total);
    }
    total
}

fn get_dir_sizes(node: &SharedNode<FsNode<'_>>) -> u64 {
    let mut total = 0;
    let NodeT::Dir { size: Some(size) } = node.borrow().value.ty else {
        unimplemented!()
    };
    if size <= 100000 {
        total += size;
    }
    for child in Node::iter_children(node) {
        if let NodeT::Dir { .. } = child.borrow().value.ty {
            total += get_dir_sizes(&child);
        }
    }
    total
}

fn large_dirs(node: &SharedNode<FsNode<'_>>) -> Vec<u64> {
    let mut out = Vec::new();
    large_dirs_internal(node, &mut out);
    out
}

fn large_dirs_internal(node: &SharedNode<FsNode<'_>>, acc: &mut Vec<u64>) {
    let NodeT::Dir { size: Some(size) } = node.borrow().value.ty else {
        unimplemented!()
    };
    #[cfg(test)]
    if size >= 8381165 {
        acc.push(size);
    }
    #[cfg(not(test))]
    if size >= 7442399 {
        acc.push(size);
    }
    for child in Node::iter_children(node) {
        if let NodeT::Dir { .. } = child.borrow().value.ty {
            large_dirs_internal(&child, acc);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct FsNode<'a> {
    ty: NodeT,
    label: &'a str,
}

impl<'a> FsNode<'a> {
    fn new_root() -> Self {
        Self {
            ty: NodeT::Dir { size: None },
            label: "#",
        }
    }

    fn from_str(s: &'a str) -> anyhow::Result<Self> {
        let (pre, label) = s.split_once(' ').unwrap();
        if pre == "dir" {
            Ok(Self {
                ty: NodeT::Dir { size: None },
                label,
            })
        } else {
            Ok(Self {
                ty: NodeT::File {
                    size: pre.parse().unwrap(),
                },
                label,
            })
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NodeT {
    File { size: u64 },
    Dir { size: Option<u64> },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cmd<'a> {
    Cd(&'a str),
    CdUp,
    Ls,
}

impl<'a> Cmd<'a> {
    fn from_str(s: &'a str) -> anyhow::Result<Self> {
        let mut pts = s.split_whitespace().skip(1);
        match pts.next() {
            Some("cd") => {
                let arg = pts.next().unwrap();
                if arg == ".." {
                    Ok(Self::CdUp)
                } else {
                    Ok(Self::Cd(arg))
                }
            }
            Some("ls") => Ok(Self::Ls),
            _ => unimplemented!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut tree: Tree<FsNode> = Tree::new();
    let root = tree.set_root(FsNode::new_root());
    let mut node = root.clone();
    let mut reading_cmds = true;
    let mut lines = input.lines().skip(1).peekable();
    while let Some(line) = lines.next() {
        if reading_cmds {
            let cmd = Cmd::from_str(line).unwrap();
            match cmd {
                Cmd::Cd(dir) => {
                    let child = Node::iter_children(&node)
                        .find(|c| c.borrow().value.label == dir)
                        .unwrap();

                    node = child;
                }
                Cmd::CdUp => {
                    node = Node::get_parent(&node).unwrap();
                }
                Cmd::Ls => reading_cmds = false,
            }
        } else {
            if let Some(next_line) = lines.peek()
                && next_line.starts_with("$")
            {
                reading_cmds = true;
            }
            Node::add_child(&node, FsNode::from_str(line).unwrap());
        }
    }
    fill_sizes(&root);

    Some(get_dir_sizes(&root))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut tree: Tree<FsNode> = Tree::new();
    let root = tree.set_root(FsNode::new_root());
    let mut node = root.clone();
    let mut reading_cmds = true;
    let mut lines = input.lines().skip(1).peekable();
    while let Some(line) = lines.next() {
        if reading_cmds {
            let cmd = Cmd::from_str(line).unwrap();
            match cmd {
                Cmd::Cd(dir) => {
                    let child = Node::iter_children(&node)
                        .find(|c| c.borrow().value.label == dir)
                        .unwrap();

                    node = child;
                }
                Cmd::CdUp => {
                    node = Node::get_parent(&node).unwrap();
                }
                Cmd::Ls => reading_cmds = false,
            }
        } else {
            if let Some(next_line) = lines.peek()
                && next_line.starts_with("$")
            {
                reading_cmds = true;
            }
            Node::add_child(&node, FsNode::from_str(line).unwrap());
        }
    }
    fill_sizes(&root);
    large_dirs(&root).iter().min().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(95437));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24933642));
    }
}
