fn md5(input: &[u8]) -> [u8; 16] {
    #![allow(non_snake_case)]

    let s: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
        5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    let K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
        0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
        0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
        0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
        0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
        0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
        0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
        0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];

    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;

    let orig_len = input.len();
    let mut input = input.to_vec();
    input.push(0x80);
    while input.len() % 64 != 56 {
        input.push(0x00);
    }

    let orig_len_bits = ((orig_len as u128 * 8) % 2u128.pow(64)) as u64;
    input.extend(orig_len_bits.to_le_bytes());

    for chunk in input.chunks(512 / 8) {
        let mut M: [u32; 16] = [0; 16];
        for (i, word) in chunk.chunks(32 / 8).enumerate() {
            let word = u32::from_le_bytes([word[0], word[1], word[2], word[3]]);
            M[i] = word;
        }

        let mut A = a0;
        let mut B = b0;
        let mut C = c0;
        let mut D = d0;

        for i in 0..=63 {
            let mut F;
            let g;
            match i {
                0..=15 => {
                    F = (B & C) | ((!B) & D);
                    g = i;
                }
                16..=31 => {
                    F = (D & B) | ((!D) & C);
                    g = (5usize.wrapping_mul(i).wrapping_add(1)) % 16;
                }
                32..=47 => {
                    F = B ^ C ^ D;
                    g = (3usize.wrapping_mul(i).wrapping_add(5)) % 16;
                }
                48..=63 => {
                    F = C ^ (B | !D);
                    g = (7usize.wrapping_mul(i)) % 16;
                }
                _ => unreachable!(),
            }

            F = F.wrapping_add(A).wrapping_add(K[i]).wrapping_add(M[g]);
            A = D;
            D = C;
            C = B;
            B = B.wrapping_add(F.rotate_left(s[i]));
        }

        a0 = a0.wrapping_add(A);
        b0 = b0.wrapping_add(B);
        c0 = c0.wrapping_add(C);
        d0 = d0.wrapping_add(D);
    }

    let a0 = a0.to_le_bytes();
    let b0 = b0.to_le_bytes();
    let c0 = c0.to_le_bytes();
    let d0 = d0.to_le_bytes();
    let mut digest: [u8; 16] = [0; 16];
    for i in 0..16 {
        match i {
            0..=3 => digest[i] = a0[i % 4],
            4..=7 => digest[i] = b0[i % 4],
            8..=11 => digest[i] = c0[i % 4],
            12..=15 => digest[i] = d0[i % 4],
            _ => unreachable!(), 
        }
    }

    digest
}

fn md5_hex(input: &[u8]) -> String {
    md5(input)
        .into_iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
        //.into_bytes()
}

fn mine(key: &str, zeroes: usize) -> u32 {
    let zeroes = "0".repeat(zeroes);
    for n in 1.. {
        let bytes = format!("{}{}", key, n).into_bytes();
        let hash = md5_hex(&bytes);

        if hash.starts_with(&zeroes) {
            return n;
        }
    }
    unreachable!()
}

fn main() {
    let input = "ckczppom";

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    println!("{}", mine(input, 5));
}

fn part2(input: &str) {
    println!("{}", mine(input, 6));
}
