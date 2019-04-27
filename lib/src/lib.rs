use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

struct Cipher<'a> {
	map1: &'a str,
	map2: &'a str,
	hash: HashMap<char, char>,
}

impl<'a> Cipher<'a> {
	fn new(map1: &'a str, map2: &'a str) -> Cipher<'a> {
		Cipher {
			map1,
			map2,
			hash: map1
				.to_lowercase()
				.chars()
				.zip(map2.to_lowercase().chars())
				.collect(),
		}
	}

	fn encode(&self, string: &str, private: bool) -> String {
		let mut result: String = String::new();
		'outer: for i in string.to_lowercase().chars() {
			for (x, y) in self.hash.iter() {
				if i == *x {
					result.push(*y);
					continue 'outer;
				}
			}
			result.push(i);
		}
		if !private {
			println!(
				"Key: {}={}\nEncode input: {}\nEncode output: {}",
				self.map1, self.map2, string, result
			)
		}
		result
	}

	fn decode(&self, string: &str, private: bool) -> String {
		let mut result: String = String::new();
		'outer: for i in string.to_lowercase().chars() {
			for (x, y) in self.hash.iter() {
				if i == *y {
					result.push(*x);
					continue 'outer;
				}
			}
			result.push(i);
		}
		if !private {
			println!(
				"Key: {}={}\nDecode input: {}\nDecode output: {}",
				self.map1, self.map2, string, result
			)
		}
		result
	}
}

fn scramble(input: &str, private: bool) -> String {
	let mut rng = rand::thread_rng();
	let mut x: Vec<char> = input.chars().collect();
	x.shuffle(&mut rng);
	let y: String = x.into_iter().collect();
	if !private {
		println!("Key: {}={}", input, y)
	}
	y
}

pub struct User {}

impl User {
	pub fn start() {
		let user = User {};
		user.selector();
	}

	fn selector(&self) {
		loop {
			println!("Encode or Decode?");
			let mut ciph_type = String::new();
			io::stdin().read_line(&mut ciph_type).expect("Line error");

			match ciph_type.to_lowercase().trim() {
				"e" | "encode" | "en" => break self.user_encoder(),
				"d" | "decode" | "de" => break self.user_decoder(),
				_ => println!("Invalid option... Try again"),
			}
		}
	}

	fn file_open(&self) -> Option<std::fs::File> {
		println!("Input name for text file... Or 'no' to not use a file");
		let mut file_name = String::new();
		io::stdin().read_line(&mut file_name).expect("Line error");
		let file_name = format!("{}.txt", file_name.trim());

		if file_name.to_lowercase() == "no.txt" {
			None
		} else {
			let file = std::fs::OpenOptions::new()
				.append(true)
				.create(true)
				.open(&file_name)
				.expect("Fail to open or create file");
			println!("Created {}", &file_name);
			Some(file)
		}
	}

	fn user_encoder(&self) {
		let file = self.file_open();
		let private = file.is_some();
		let (map1, map2);
		let mut key;

		let cipher = loop {
			println!("Would you like to insert your own key?");
			let mut choice = String::new();
			io::stdin().read_line(&mut choice).expect("Line error");

			match choice.to_lowercase().trim_end() {
				"no" | "n" => {
					map1 = "abcdefghijklmnopqrstuvwxyz";
					map2 = scramble(&map1, true);
					break Cipher::new(map1, &map2);
				}
				"yes" | "y" => {
					println!("Input key (format: abc=xyz)");
					key = String::new();

					io::stdin().read_line(&mut key).expect("Line error");

					let key_vec: Vec<&str> = key.trim().split('=').collect();
					if key_vec[0].len() != key_vec[1].len() {
						println!("Key error: unequal lengths");
						continue;
					}

					break Cipher::new(key_vec[0], key_vec[1]);
				}
				_ => println!("Invalid option... Try again"),
			}
		};

		println!("Input data for encryption... Press enter twice to finish");
		let mut data = String::new();

		while !data.contains("\n\n") && !data.contains("\r\n\r\n") {
			io::stdin().read_line(&mut data).expect("Line error");
		}
		data = data.trim().to_string();

		let trans = cipher.encode(&data, private);
		if private {
			write!(
				&mut file.unwrap(),
				"--\n{}\n-Key: {}={}-\n{}\n--\n",
				data,
				cipher.map1,
				cipher.map2,
				trans
			)
			.expect("Fail to write file");
		}
	}

	fn user_decoder(&self) {
		let file = self.file_open();
		let private = file.is_some();
		let mut key;

		let cipher = loop {
			println!("Input key (format: abc=xyz)");
			key = String::new();

			io::stdin().read_line(&mut key).expect("Line error");

			let key_vec: Vec<&str> = key.trim().split('=').collect();
			if key_vec[0].len() != key_vec[1].len() {
				println!("Key error: unequal lengths");
				continue;
			}
			break Cipher::new(key_vec[0], key_vec[1]);
		};

		println!("Input encoded data... Press enter twice to finish");
		let mut data = String::new();

		while !data.contains("\n\n") && !data.contains("\r\n\r\n") {
			io::stdin().read_line(&mut data).expect("Line error");
		}
		data = data.trim().to_string();

		let trans = cipher.decode(&data, true);
		if private {
			write!(
				&mut file.unwrap(),
				"--\n{}\n-Key: {}={}-\n{}\n--\n",
				data,
				cipher.map1,
				cipher.map2,
				trans
			)
			.expect("Fail to write file");
		}
	}
}
