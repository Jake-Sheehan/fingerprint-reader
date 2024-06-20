use std::fmt;
use std::fs;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Fingerprint {
    data: Vec<Vec<char>>,
    name: String,
    year: u32,
    rows: usize,
    cols: usize,
}

#[allow(dead_code)]
impl Fingerprint {
    fn read_file_lines(filename: &str) -> Vec<String> {
        let uri = format!("data/{}", filename);
        fs::read_to_string(uri)
            .expect("error: failed to read file")
            .lines()
            .map(String::from)
            .collect()
    }

    pub fn new(filename: &str) -> Self {
        let lines = Self::read_file_lines(filename);
        let name = lines[0].trim();
        let name: String = name.to_string();
        let year: u32 = lines[1].trim().parse().unwrap();
        let rows: usize = lines[2].trim().parse().unwrap();
        let cols: usize = lines[3].trim().parse().unwrap();
        let data: Vec<Vec<char>> = lines
            .iter()
            .skip(4)
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        return Self {
            data,
            name,
            year,
            rows,
            cols,
        };
    }

    pub fn get_number_of_pixels(&self) -> usize {
        self.rows * self.cols
    }

    pub fn print_image(&self) {
        for line in &self.data {
            for c in line {
                print!("{}", c);
            }
            print!("\n");
        }
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Fingerprint for: {}. Year registered: {}. Number of pixels: {}",
            self.name,
            self.year,
            self.get_number_of_pixels()
        )
    }
}

impl PartialEq for Fingerprint {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name
            || self.year != other.year
            || self.rows != other.rows
            || self.cols != other.cols
        {
            return false;
        }

        let epsilon: f32 = 0.1;
        let mut matches: u32 = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.data[row][col] == other.data[row][col] {
                    matches += 1;
                }
            }
        }

        let accuracy: f32 = (matches as f32) / (self.get_number_of_pixels() as f32);

        if accuracy > 1.0 - epsilon {
            return true;
        } else {
            return false;
        }
    }
}

impl Eq for Fingerprint {}
