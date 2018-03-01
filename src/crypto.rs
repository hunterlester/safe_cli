use tiny_keccak::Keccak;

pub fn sha3_hash(data: String) -> [u8; 32] {
  let mut sha3 = Keccak::new_sha3_256();
  let data: Vec<u8> = From::from(data);
  sha3.update(&data);
  let mut res: [u8; 32] = [0; 32];
  sha3.finalize(&mut res);
  res
}
