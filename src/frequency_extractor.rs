use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Frequencies(pub HashMap<char, u32>);

impl Frequencies {
    pub fn new(mut file: File) -> Self {
        let mut frequencies = HashMap::new();

        let mut buffer = [0u8; 128];

        loop {
            let size = file.read(&mut buffer).expect("Failed to read file");

            if size == 0 {
                break;
            }

            let str = std::str::from_utf8(&buffer[..size]).expect("Invalid UTF-8");

            for c in str.chars() {
                frequencies
                    .entry(c)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        Self(frequencies)
    }
}
