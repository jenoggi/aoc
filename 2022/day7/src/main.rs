use std::io;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

//NOTE this is probably more complicated than necessary 
//but I wanted to try out a few things

//also, I should add some more comments

fn read_strings_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

struct FileNode {
    name: String,
	parent_dir: usize,
    size: u64
}

struct DirNode {
    name: String,
	parent_dir: usize,
	children: Vec<usize>
}

enum Node {
	File(FileNode),
	Dir(DirNode)
}

struct FileTree {
    nodes: Vec<Node>,
	current_dir: usize,
	current_pos: usize,
}

impl Default for FileTree {
    fn default() -> FileTree {
        FileTree {
			// we add root directory
            nodes: vec![Node::Dir( DirNode { name: "/".to_string(),
		                                     parent_dir: 0,
	                                         children: vec![]
	                                       })],
            current_dir: 0,
			current_pos: 1
        }
    }
} 

impl FileTree {
    fn cd(&mut self, dir: String) {
		if dir == "/" {
			self.current_dir = 0;
		} else {
			match &self.nodes[self.current_dir] {
				Node::Dir(d) => {
					if dir == "..".to_string() {
						self.current_dir = d.parent_dir;
						return;
					} else {
						for c in &d.children {
							match &self.nodes[*c] {
								Node::Dir(cd) => {
									if cd.name == dir {
										self.current_dir = *c;
										return;
									}
								}
								_ => { }
							}
						}
					}
				}
				_ => { }
			}
		// couldn't find directory
		println!("directory {} not found", dir);
		}
    }
	
	fn add(&mut self, node_name: String, node_size: u64, is_dir: bool) {
		if is_dir {
			let new_dir = DirNode {
					name: node_name,
					parent_dir: self.current_dir,
					children: vec![]
			};
			self.nodes.push(Node::Dir(new_dir));
		} else {
			let new_file = FileNode {
					name: node_name,
					parent_dir: self.current_dir,
					size: node_size
			};
			self.nodes.push(Node::File(new_file));
		}
		match &mut self.nodes[self.current_dir] {
			Node::Dir(d) => {
				d.children.push(self.current_pos);
				self.current_pos = self.current_pos + 1;
			}
			_ => { println!("Don't Panic"); }
		}
	}
	
	fn print(&self) {
		self.print_intern(0,0);
	}
	
	fn print_intern(&self, pos: usize, indentation: u64) {
		match &self.nodes[pos] {
			Node::Dir(d) => {
				for _i in 0..indentation {print!(" ");}
				println!("{}",d.name);
				for c in &d.children {
					self.print_intern(*c, indentation + 2);
				}
			}
			Node::File(d) => {
				for _i in 0..indentation {print!(" ");}
				println!("{} {}",d.name, d.size);
			}
		}
	}
	
	fn size(&self) -> u64 {
		self.size_intern(self.current_dir)
	}
	
	fn size_intern(&self, pos: usize) -> u64 {
		match &self.nodes[pos] {
			Node::Dir(d) => {
				let mut size = 0;
				/*for _i in 0..d.children.len() {
					size = size + self.size_intern(d.children[_i]);
				}*/
				for c in &d.children {
					size = size + self.size_intern(*c);
				}
				return size;
			}
			Node::File(d) => {
				return d.size;
			}
		}
	}
	
	//note I added the searches for the puzzles here for easier access
	fn puzzle_a(&self) -> u64 {
		let mut ret = 0;
		for i in 0..self.nodes.len() {
			match &self.nodes[i] {
				Node::Dir(_d) => {
					let dir_size = self.size_intern(i);
					if dir_size <= 100000 {
						ret = ret + dir_size;
					}
				}
				_ => {}
			}
		}
		ret
	}

	fn puzzle_b(&self) -> u64 {
		let mut min_space_freed = self.size();
		let space_needed = min_space_freed - 40000000;
		println!("need to free at least {} ", space_needed);
		for i in 0..self.nodes.len() {
			match &self.nodes[i] {
				Node::Dir(_d) => {
					let dir_size = self.size_intern(i);
					if (dir_size >= space_needed) && (dir_size < min_space_freed) {
						min_space_freed = dir_size;
					}
				}
				_ => {}
			}
		}
		min_space_freed
	}
}

fn main() {
	let iv = read_strings_from_file("day7_input.txt").unwrap();
	let mut fs : FileTree = Default::default();
	
	for is in iv {
		if is.starts_with('$') {
			if &is[2..4] == "cd" {
				let dir = &is[5..is.len()];
				fs.cd(dir.to_string());
			}
			// Note! I don't care about the ls commands here
		} else {
			let ts = is.split_whitespace().collect::<Vec<&str>>();
			let is_dir = ts[0] == "dir";
			let mut node_size = 0;
			if !is_dir {
				for c in ts[0].chars() {
					match c {
						'0'..='9' => {
							node_size = node_size*10 + c.to_digit(10).unwrap() as u64;
						}
						_ => { println!("Don't Panic {}",c); }
					}
				}
			}
			fs.add(ts[1].to_string(), node_size, is_dir);
		}
	}
	fs.print();
	
	fs.cd("/".to_string());
	
    println!("Your result is {} {}", fs.puzzle_a(), fs.puzzle_b());
}
