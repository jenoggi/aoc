use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::str::FromStr;

struct Monkey {
	times_inspected: i64,
    items: Vec<i64>,
    op: i64,
	test: i64,
	true_outcome: usize,
	false_outcome: usize,
}

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
	
	let lines: Vec<String> = br.lines()
        .map(|l| l.expect("Could not parse line"))
		.filter(|l| !l.is_empty())
        .collect();
	Ok(lines)
}

fn get_i64_vec_from_string(str: &String) -> Vec<i64> {
	str.replace(",","")
	   .split_whitespace()
	   .filter_map(|n| i64::from_str(n).ok())
	   .collect()
}

fn main() -> Result<(), Error> {
	// day11b.txt -> input from example
	// day11a.txt -> puzzle input
	//let seq = read(File::open("day11b.txt")?)?;
    let seq = read(File::open("day11a.txt")?)?;
	let mut monkey_list: Vec<Monkey> = Vec::new();
	let mut greatest_common_product = 1;
	
	for i in 0..seq.len()/6 {
		let items = get_i64_vec_from_string(&seq[i*6+1]);
		
		let mut op = get_i64_vec_from_string(&seq[i*6+2]);
		if op.is_empty() {
			//mark power
			op.push(0);
		} else if seq[i*6+2].contains("+") {
			//mark addition 
			op[0] = -op[0];
		}
		
		let test = get_i64_vec_from_string(&seq[i*6+3]);
		
		let true_outcome = get_i64_vec_from_string(&seq[i*6+4]);
		
		let false_outcome = get_i64_vec_from_string(&seq[i*6+5]);
		
		monkey_list.push(Monkey {times_inspected: 0, items: items, op: op[0], test: test[0],
		                         true_outcome: true_outcome[0] as usize,
								 false_outcome: false_outcome[0] as usize});
		
		//Note: could find common divisor to get a smaller number
		//seems not worth the effor though
		greatest_common_product *= test[0];
	}
	
	// part 1:
	//for _i in 0..20 {
	for _i in 0..10000 {
		for j in 0..monkey_list.len() {
			if monkey_list[j].op > 0 {
				for k in 0..monkey_list[j].items.len() {
					monkey_list[j].times_inspected += 1;
					monkey_list[j].items[k] *= monkey_list[j].op;
					monkey_list[j].items[k] = monkey_list[j].items[k]%greatest_common_product;
					// part 1:
					//monkey_list[j].items[k] /= 3;
				}
			} else if monkey_list[j].op < 0 {
				for k in 0..monkey_list[j].items.len() {
					monkey_list[j].times_inspected += 1;
					monkey_list[j].items[k] -= monkey_list[j].op;
					monkey_list[j].items[k] = monkey_list[j].items[k]%greatest_common_product;
					// part 1:
					//monkey_list[j].items[k] /= 3;
				}
			} else {
				for k in 0..monkey_list[j].items.len() {
					monkey_list[j].times_inspected += 1;
					monkey_list[j].items[k] = monkey_list[j].items[k]*monkey_list[j].items[k];
					monkey_list[j].items[k] = monkey_list[j].items[k]%greatest_common_product;
					// part 1:
					//monkey_list[j].items[k] /= 3;
				}
			}
			while monkey_list[j].items.len()>0 {
				let item = monkey_list[j].items.pop().unwrap();
				let new_pos;
				if (item%monkey_list[j].test)==0 {
					new_pos = monkey_list[j].true_outcome;
				} else {
					new_pos = monkey_list[j].false_outcome;
				}
				monkey_list[new_pos].items.push(item);
			}
		}
	}
	
	let mut greatest = 0;
	let mut next_greatest = 0;
	for i in 0..monkey_list.len() {
		if greatest < monkey_list[i].times_inspected {
			next_greatest = greatest;
			greatest = monkey_list[i].times_inspected;
		} else if next_greatest < monkey_list[i].times_inspected {
			next_greatest = monkey_list[i].times_inspected;
		}
	}
	println!("  Your answer is {} ",next_greatest*greatest);
	Ok(()) 
}
