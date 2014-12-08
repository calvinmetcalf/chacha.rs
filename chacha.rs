#![allow(experimental)]
use std::simd::u32x4;
fn main() {
     
    let mut a = ChaCha20::new([0u32, ..8],[0u32, ..3]);
    let b = a.next();
    println!("{}", b);
}
// fn add_arr(a:u32x4, b: u32x4) -> u32x4{
//   return a+b;
// }
fn rotate(v:u32x4, c: u32x4) -> u32x4{
  let s32 = u32x4(32, 32, 32, 32);
  let r = s32 - c;
  let right = v >> r;
  return (v << c) ^ right;
}
struct Base {
  key: [u32, ..8],
  nonce: [u32, ..3],
}
struct ChaCha20 {
  base: Base,
  index: u32
}
impl ChaCha20 {
  fn new(key: [u32, ..8], nonce: [u32, ..3]) -> ChaCha20 {
    let base = Base {
      key: key,
      nonce: nonce
    };
    ChaCha20 {
      base: base,
      index: 0u32
    }
  }
 }
impl Iterator<[u32,..16]> for ChaCha20 {
  fn next(&mut self) -> Option<[u32,..16]> {
    let out = chacha_block(self.base, self.index);
    self.index = self.index + 1;
    return Some(out);
  }
}
#[deriving(Show)]
struct ChaCha {
  a: u32x4,
  b: u32x4,
  c: u32x4,
  d: u32x4
}
// impl ChaCha {
//     fn clone(&self) -> ChaCha {
//         ChaCha {
//           a: self.a,
//           b: self.b,
//           c: self.c,
//           d: self.d
//         }
//     }
// }
fn chacha_block(keys: Base, counter: u32) -> [u32,..16]{
  let input = ChaCha {
    a: u32x4(1634760805, 857760878, 2036477234, 1797285236),
    b: u32x4(keys.key[0], keys.key[1], keys.key[2], keys.key[3]),
    c: u32x4(keys.key[4], keys.key[5], keys.key[6], keys.key[7]),
    d: u32x4(counter, keys.nonce[0], keys.nonce[1], keys.nonce[2])
  };
  let mut state = ChaCha {
    a: u32x4(1634760805, 857760878, 2036477234, 1797285236),
    b: u32x4(keys.key[0], keys.key[1], keys.key[2], keys.key[3]),
    c: u32x4(keys.key[4], keys.key[5], keys.key[6], keys.key[7]),
    d: u32x4(counter, keys.nonce[0], keys.nonce[1], keys.nonce[2])
  };
  let mut i = 0u;
  loop {
    i = i + 1;
    if i > 10 {
      break;
    }
    round(&mut state);
    let u32x4(b10, b11, b12, b13) = state.b;
    state.b = u32x4(b11, b12, b13, b10);
    let u32x4(c10, c11, c12, c13) = state.c;
    state.c = u32x4(c12, c13,c10, c11);
    let u32x4(d10, d11, d12, d13) = state.d;
    state.d = u32x4(d13, d10, d11, d12);
    round(&mut state);
    let u32x4(b20, b21, b22, b23) = state.b;
    state.b = u32x4(b23, b20, b21, b22);
    let u32x4(c20, c21, c22, c23) = state.c;
    state.c = u32x4(c22, c23, c20, c21);
    let u32x4(d20, d21, d22, d23) = state.d;
    state.d = u32x4(d21, d22, d23, d20);
  }
  let u32x4(a1, a2, a3, a4) = input.a + state.a;
  let u32x4(b1, b2, b3, b4) = input.b + state.b;
  let u32x4(c1, c2, c3, c4) = input.c + state.c;
  let u32x4(d1, d2, d3, d4) = input.d + state.d;
  return [
    a1,a2,a3,a4,
    b1,b2,b3,b4,
    c1,c2,c3,c4,
    d1,d2,d3,d4
  ];
}

fn round(state: &mut ChaCha) -> () {
  let s16 = u32x4(16, 16, 16, 16);
  let s12 = u32x4(12, 12, 12, 12);
  let s8 = u32x4(8, 8, 8, 8);
  let s7 = u32x4(7, 7, 7, 7);
  
  state.a = state.a + state.b;
  state.d = rotate(state.d ^ state.a, s16);
  state.c = state.c + state.d;
  state.b = rotate(state.b ^ state.c, s12);
  state.a = state.a + state.b;
  state.d = rotate(state.d ^ state.a, s8);
  state.c = state.c + state.d;
  state.b = rotate(state.b ^ state.c, s7);
}
