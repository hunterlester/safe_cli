use tiny_keccak::Keccak;
use std::io;

pub fn sha3_hash() -> [u8; 32] {
  println!("Please enter your secret:");
  let mut data = String::new();
  io::stdin().read_line(&mut data).expect("Please enter valid string");
  data = data.trim().to_string();
  let mut sha3 = Keccak::new_sha3_256();
  let data: Vec<u8> = From::from(data);
  sha3.update(&data);
  let mut res: [u8; 32] = [0; 32];
  sha3.finalize(&mut res);
  res
}
