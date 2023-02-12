use std::fs::File;
use std::io::Read;


pub struct Key {
        data: Vec<u8>,
}


impl Key {
        pub fn new(size: usize) -> Key {

                let mut file = File::open("/dev/random").unwrap();
                let mut data = vec![0; size];

                file.read_exact(&mut data).unwrap();
                Key { data }
        }


        pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
                data
                        .iter()
                        .zip(self.data.iter().cycle())
                        .map(|(a, b)| a ^ b)
                        .collect()
        }


        pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
                self.encrypt(data)
        }
}
