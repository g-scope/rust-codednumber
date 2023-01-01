use rand::distributions::{Alphanumeric, DistString};

struct CodedNumber {
    data: Vec<u8>,
    key: String
}

impl CodedNumber {
    fn new(starting_data: isize) -> CodedNumber {
        let mut new_coded_number = CodedNumber {
            data: Vec::new(),
            key: Alphanumeric.sample_string(&mut rand::thread_rng(), 10)
        };

        new_coded_number.set_data(starting_data);

        new_coded_number
    }

    fn set_data(&mut self, target_number: isize) {
        let new_key: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);
        let key_bytes = new_key.as_bytes();

        let mut location: usize = 0;

        let data_bytes = isize::to_be_bytes(target_number);
        let data_length = data_bytes.len();

        let mut vec_of_bytes = Vec::new();

        while location < data_length {
            vec_of_bytes.insert(
                vec_of_bytes.len(),
                data_bytes[location] ^ key_bytes[location]
            );
            location += 1
        }

        self.data = vec_of_bytes;
        self.key = new_key;
    }

    fn get_data(&self) -> isize {
        let mut byte_array = [0 as u8; 8];

        let data_length = self.data.len();
        let key_bytes = self.key.as_bytes();

        let mut location = 0;

        while location < data_length {
            byte_array[location] = self.data[location] ^ key_bytes[location];
            location += 1
        }

        isize::from_be_bytes(byte_array)
    }
}

fn main() {
    let mut test = CodedNumber::new(-5);

    loop {
        println!("XOR'ed Bytes: {:#?}\nXOR Key: {}", test.data, test.key);
        println!("Number: {}\n\nUpdate Number To...", test.get_data());
        
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        match line.trim_end().parse::<isize>() {
            Ok(n) => { test.set_data(n); println!("Success!") },
            Err(_e) => println!("Not a number!")
        }
    }
}
