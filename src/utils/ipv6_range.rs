use ipnet::Ipv6Net;
use rand::{
  prelude::{thread_rng, SeedableRng, SmallRng},
  Rng,
};
use std::net::IpAddr;

/// # IPv6Range
/// 我知道，我代码写得很屎，不会搞比较高级的位操作，只好土法炼钢了。
///
/// 这个对象从一个 Ipv6Net 构建，并提供名为 `get_random_ip` 的方法来从这个 IP 段随机获取一个 IP。
pub struct IPv6Range {
  prefix_length: usize,
  network_str: String,
  rng: SmallRng,
}

impl IPv6Range {
  pub fn new(ipv6_net: Ipv6Net) -> IPv6Range {
    let mut thread_rng = thread_rng();
    IPv6Range {
      prefix_length: ipv6_net.prefix_len() as usize,
      network_str: format!("{}", ipv6_net.network()).replace(":", ""),
      rng: SmallRng::from_rng(&mut thread_rng).unwrap(),
    }
  }
  fn get_bit_ary_from_letter(letter: char) -> [bool; 4] {
    match letter {
      '0' => [false, false, false, false],
      '1' => [false, false, false, true],
      '2' => [false, false, true, false],
      '3' => [false, false, true, true],
      '4' => [false, true, false, false],
      '5' => [false, true, false, true],
      '6' => [false, true, true, false],
      '7' => [false, true, true, true],
      '8' => [true, false, false, false],
      '9' => [true, false, false, true],
      'a' => [true, false, true, false],
      'b' => [true, false, true, true],
      'c' => [true, true, false, false],
      'd' => [true, true, false, true],
      'e' => [true, true, true, false],
      'f' => [true, true, true, true],
      _ => [false, false, false, false],
    }
  }
  fn get_letter_from_bit_ary(bit_ary: [bool; 4]) -> char {
    match bit_ary {
      [false, false, false, false] => '0',
      [false, false, false, true] => '1',
      [false, false, true, false] => '2',
      [false, false, true, true] => '3',
      [false, true, false, false] => '4',
      [false, true, false, true] => '5',
      [false, true, true, false] => '6',
      [false, true, true, true] => '7',
      [true, false, false, false] => '8',
      [true, false, false, true] => '9',
      [true, false, true, false] => 'a',
      [true, false, true, true] => 'b',
      [true, true, false, false] => 'c',
      [true, true, false, true] => 'd',
      [true, true, true, false] => 'e',
      [true, true, true, true] => 'f',
    }
  }
  fn get_bit_ary(&self) -> [bool; 128] {
    let mut bit_ary = [false; 128];
    let chars: Vec<char> = self.network_str.chars().collect();
    for i in 0..chars.len() {
      let slice = &mut bit_ary[i * 4..i * 4 + 4];
      let bit_slice = IPv6Range::get_bit_ary_from_letter(chars[i]);
      [slice[0], slice[1], slice[2], slice[3]] = bit_slice;
    }
    return bit_ary;
  }
  fn get_random_ip_bit_ary(&mut self) -> [bool; 128] {
    let mut bit_ary = self.get_bit_ary();
    let mut rand_ary = [false; 128];
    self.rng.fill(&mut rand_ary);
    for i in self.prefix_length..bit_ary.len() {
      bit_ary[i] = rand_ary[i];
    }
    return bit_ary;
  }
  pub fn get_random_ip(&mut self) -> IpAddr {
    let bit_ary = self.get_random_ip_bit_ary();
    let mut string = String::new();
    for i in 0..32 {
      let j = i * 4;
      let letter = IPv6Range::get_letter_from_bit_ary([
        bit_ary[j],
        bit_ary[j + 1],
        bit_ary[j + 2],
        bit_ary[j + 3],
      ]);
      string.push(letter);
      if (i % 4 == 3) && (i != 31) {
        string.push(':')
      }
    }
    return IpAddr::V6(string.parse().unwrap());
  }
}
