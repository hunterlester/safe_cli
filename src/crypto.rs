//use tiny_keccak::Keccak;
//use helpers::{ read_line };
//use console::style;
//
//pub fn sha3_hash() -> Option<[u8; 32]> {
//  println!("{}", style("Please enter data to be hashed:").yellow().bold());
//  let mut data = String::new();
//  data = read_line(&mut data);
//  let mut sha3 = Keccak::new_sha3_256();
//  let data: Vec<u8> = From::from(data);
//  sha3.update(&data);
//  let mut res: [u8; 32] = [0; 32];
//  sha3.finalize(&mut res);
//  Some(res)
//}
