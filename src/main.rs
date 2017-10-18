extern crate collections_test;

use std::io;
use std::io::prelude::*;

use collections_test::*;

fn main() {
	let list = IntList::new((0..11).collect());
	println!("{:?}", list);
	println!("{}", list.mean());
	println!("{}", list.median());
	println!("{}", list.mode());

	println!("----------------------");

	println!("{:?}", pig_latin("something"));

	println!("----------------------");

	let mut company = Company::new(Box::new(|employees: &Vec<String>|{
		println!("{} employees to display.", employees.len());
		for employee in employees.iter() {
			println!("{:?}", employee);
		}
	}));
	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		company.process(line.unwrap());
		println!("{}", company);
	}
}