const BITS: usize = 12;

fn part1(input: &[u32]) {
    let counts = input.iter().fold([0; BITS], |mut counts, n| {
        for i in 0..BITS {
            counts[i] += (n >> i) & 1;
        }
        counts
    });
    let min = input.len() as u32 / 2 + 1;
    let mut gamma = 0;
    for i in 0..BITS {
        let bit = if counts[i] >= min { 1 } else { 0 };
        gamma |= bit << i;
    }

    // invert and keep the lower `BITS` bits
    let epsilon = (!gamma) & (2u32.pow(BITS as u32) - 1);

    println!("{}", gamma * epsilon);
}

fn part2(input: &[u32]) {
    let mut oxg_nums = input.to_vec();
    let mut oxg_idx = BITS - 1;
    while oxg_nums.len() > 1 {
        let count1 = oxg_nums.iter().fold(0, |count, n| count + ((n >> oxg_idx) & 1));
        let count0 = oxg_nums.len() as u32 - count1;
        let bit = if count1 >= count0 { 1 } else { 0 };
        oxg_nums.retain(|n| ((n >> oxg_idx) & 1) == bit);
        if oxg_idx > 0 {
            oxg_idx -= 1;
        }
    }

    let mut co2_nums = input.to_vec();
    let mut co2_idx = BITS - 1;
    while co2_nums.len() > 1 {
        let count1 = co2_nums.iter().fold(0, |count, n| count + ((n >> co2_idx) & 1));
        let count0 = co2_nums.len() as u32 - count1;
        let bit = if count0 <= count1 { 0 } else { 1 };
        co2_nums.retain(|n| ((n >> co2_idx) & 1) == bit);
        if co2_idx > 0 {
            co2_idx -= 1;
        }
    }

    let value = oxg_nums.into_iter().zip(co2_nums.into_iter()).map(|(oxg, co2)| oxg * co2).next().unwrap();
    println!("{}", value);
}

fn main() {
    let input: Vec<u32> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect();

    part1(&input);
    part2(&input);
}
