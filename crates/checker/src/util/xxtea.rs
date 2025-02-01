//! XXTEA util for leet and suid modules.
//!
//! Modified from https://github.com/ret2shell/ret2script.

const DELTA: u32 = 0x9E3779B9;

fn to_bytes(v: &[u32], include_length: bool) -> Vec<u8> {
    let length: u32 = v.len() as u32;
    let mut n: u32 = length << 2;
    if include_length {
        let m: u32 = v[length as usize - 1];
        n -= 4;
        assert!(!((m < n - 3) || (m > n)));
        n = m;
    }
    let mut bytes: Vec<u8> = vec![0; n as usize];
    for i in 0..n {
        bytes[i as usize] = (v[(i >> 2) as usize] >> ((i & 3) << 3)) as u8;
    }
    bytes
}

fn to_u32(bytes: &[u8], include_length: bool) -> Vec<u32> {
    let length: u32 = bytes.len() as u32;
    let mut n: u32 = length >> 2;
    if length & 3 != 0 {
        n += 1;
    }
    let mut v;
    if include_length {
        v = vec![0; n as usize + 1];
        v[n as usize] = length;
    } else {
        v = vec![0; n as usize];
    }
    for i in 0..length {
        v[(i >> 2) as usize] |= (bytes[i as usize] as u32) << ((i & 3) << 3);
    }
    v
}

fn mx(sum: u32, y: u32, z: u32, p: u32, e: u32, k: &[u32]) -> u32 {
    ((z >> 5 ^ y << 2).wrapping_add(y >> 3 ^ z << 4))
        ^ ((sum ^ y).wrapping_add(k[(p & 3 ^ e) as usize] ^ z))
}

fn fix_k(k: &[u32]) -> Vec<u32> {
    let mut key = k.to_owned();
    if key.len() < 4 {
        let length = key.len();
        for _ in length..4 {
            key.push(0)
        }
    }
    key
}

fn encrypt_(v: &mut [u32], k: &[u32]) -> Vec<u32> {
    let length: u32 = v.len() as u32;
    let n: u32 = length - 1;
    let key: Vec<u32> = fix_k(k);
    let mut e: u32;
    let mut y: u32;
    let mut z = v[n as usize];
    let mut sum: u32 = 0;
    let mut q: u32 = 6 + 52 / length;
    while q > 0 {
        sum = sum.wrapping_add(DELTA);
        e = sum >> 2 & 3;
        for p in 0..n {
            y = v[(p as usize) + 1];
            v[p as usize] = v[p as usize].wrapping_add(mx(sum, y, z, p, e, &key));
            z = v[p as usize];
        }
        y = v[0];
        v[n as usize] = v[n as usize].wrapping_add(mx(sum, y, z, n, e, &key));
        z = v[n as usize];
        q -= 1;
    }
    v.to_owned()
}

fn decrypt_(v: &mut [u32], k: &[u32]) -> Vec<u32> {
    let length: u32 = v.len() as u32;
    let n: u32 = length - 1;
    let key: Vec<u32> = fix_k(k);
    let mut e: u32;
    let mut y: u32 = v[0];
    let mut z;
    let q: u32 = 6 + 52 / length;
    let mut sum: u32 = q.wrapping_mul(DELTA);
    while sum != 0 {
        e = sum >> 2 & 3;
        let mut p: usize = n as usize;
        while p > 0 {
            z = v[p - 1];
            v[p] = v[p].wrapping_sub(mx(sum, y, z, p as u32, e, &key));
            y = v[p];
            p -= 1;
        }
        z = v[n as usize];
        v[0] = v[0].wrapping_sub(mx(sum, y, z, 0, e, &key));
        y = v[0];
        sum = sum.wrapping_sub(DELTA);
    }
    v.to_owned()
}

pub fn encrypt_raw(data: &[u8], key: &str) -> Vec<u8> {
    let key = key.as_bytes();
    to_bytes(
        &encrypt_(&mut to_u32(data, false), &to_u32(key, false)),
        false,
    )
}

pub fn decrypt_raw(data: &[u8], key: &str) -> Vec<u8> {
    let key = key.as_bytes();
    to_bytes(
        &decrypt_(&mut to_u32(data, false), &to_u32(key, false)),
        false,
    )
}
