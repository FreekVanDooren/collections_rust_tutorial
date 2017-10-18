#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug)]
pub struct IntList {
	integers: Vec<i32>
}

impl IntList {

	pub fn new( numbers_list: Vec<i32>) -> IntList {
		IntList {
			integers: numbers_list
		}
	}

	pub fn mean( &self ) -> f64 {
		let mut total: f64 = 0.0;
		for number in self.integers.iter() {
			total += *number as f64;
		}
		total / ( self.integers.len() as f64)
	}

	pub fn median( &self ) -> i32 {
		let sorted_ints = self.get_sorted_ints();
		sorted_ints[ sorted_ints.len() / 2 ]
	}

	fn get_sorted_ints( &self ) -> Vec<i32> {
		let mut sortable_ints = self.integers.clone();
		sortable_ints.sort();
		sortable_ints
	}

	pub fn mode( &self ) -> i32 {
		let mut max_count = 0;
		let mut mode_value = self.integers[0]; // just choose the first one, who cares, at least it's in the list
		let value_counts = self.get_value_counts();
		for (value, count) in value_counts {
			if count > max_count {
				max_count = count;
				mode_value = *value;
			}
		}
		mode_value
	}

	fn get_value_counts( &self ) -> HashMap <&i32, i32> {
		let mut value_counts = HashMap::new();
		for number in self.integers.iter() {
			*value_counts.entry(number).or_insert(0) += 1;
		}
		value_counts
	}
}

pub fn pig_latin( input: &str ) -> String {
	let (postfix_char, start_index) = match &input[0..1] {
		"a" | "e" | "i" | "o" | "u" => ("h", 0),
		x => (x, 1)
	};
	let start_of_word = &input[start_index..input.len()];

	format!("{}-{}ay", start_of_word, postfix_char)
}

pub struct Company {
	departments: HashMap<String, HashSet<String>>,
	process: Box<Fn(&Vec<String>)->()>
}

impl fmt::Display for Company {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter
			.debug_struct("Company")
			.field("departments", &self.departments)
			.finish()
	}
}

impl Company {

	pub fn new(process_fn: Box<Fn(&Vec<String>)->()>) -> Company {
		Company{
			departments: HashMap::new(),
			process: process_fn
		}
	}

	pub fn process( &mut self , line: String) {
		lazy_static! {
			static ref ADD_PATTERN: Regex = Regex::new(r"^Add\s+(.+)\s+to\s+(.+)$").unwrap();
			static ref PRINT_PATTERN: Regex = Regex::new(r"^Print\s*(.*)$").unwrap();
		}
		if ADD_PATTERN.is_match(&line) {
			let captures = ADD_PATTERN.captures(&line).unwrap();
			let employee = String::from(&captures[1]);
			let department = String::from(&captures[2]);
			self.save(employee, department);
		} else if PRINT_PATTERN.is_match(&line) {
			let captures = PRINT_PATTERN.captures(&line).unwrap();
			let employees = match self.departments.get(&captures[1]).take() {
				Some(department) => department.clone(),
				None => self.all_departments()
			};
			(self.process) (&self.to_sorted(employees));
 		}
	}

	fn save(&mut self, employee: String, department: String) {
		let employees = self.departments.entry(department).or_insert(HashSet::new());
		employees.insert(employee);
	}

	fn all_departments(&self) -> HashSet<String> {
		let mut all_employees = HashSet::new();
		for employees in self.departments.values() {
			all_employees = all_employees.union(&employees).cloned().collect();
		}
		all_employees
	}

	fn to_sorted(&self, employees: HashSet<String>) -> Vec<String> {
		let mut sortable_employees: Vec<String> = Vec::new();
		for employee in employees.iter() {
			sortable_employees.push(employee.to_string());
		}
		sortable_employees.sort_by(|a,b| a.cmp(b));
		sortable_employees
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_mean() {
    	let test_list = IntList::new((0..12).collect());
    	assert_eq!(5.5, test_list.mean());
    }

    #[test]
    fn calc_median() {
    	let test_list = IntList::new(vec![4,7,2,7,3,4,8,3,5]);
    	assert_eq!(4, test_list.median());
    }

    #[test]
    fn calc_mode() {
    	let test_list = IntList::new(vec![4,7,2,7,3,4,7,8,3,5]);
    	assert_eq!(7, test_list.mode());
    }

    #[test]
    fn pig_latin_start_with_consonant() {
    	assert_eq!(String::from("irst-fay"), pig_latin("first") );
    }

    #[test]
    fn pig_latin_start_with_vowel() {
    	assert_eq!(String::from("apple-hay"), pig_latin("apple") );
    }

}