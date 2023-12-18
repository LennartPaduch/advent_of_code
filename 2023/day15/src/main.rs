fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lenses_iter = || input.trim().split(',');

    let mut hash_sum = 0u32;
    let mut cur_hash = 0u8;
    for &ch in input.as_bytes() {
        match ch {
            b',' | b'\n' => {
                hash_sum += cur_hash as u32;
                cur_hash = 0;
            }
            _ => cur_hash = cur_hash.wrapping_add(ch).wrapping_mul(17),
        }
    }
    println!("Part 1: {}", hash_sum);

    const VAL: Vec<(&str, u8)> = Vec::new();
    let mut boxes = [VAL; 256];
    lenses_iter().for_each(|lens| process_lens(lens, &mut boxes));
    let sum = boxes.iter().enumerate().fold(0, |acc, (i, lenses)| {
        acc + lenses
            .iter()
            .enumerate()
            .fold(0, |acc, (j, (_, focal_length))| {
                acc + (i + 1) * (j + 1) * (*focal_length as usize)
            })
    });
    println!("Part 2: {}", sum);
}

fn process_lens<'a>(lens: &'a str, boxes: &mut [Vec<(&'a str, u8)>; 256]) {
    let (label, operation, focal_length) = if let Some(idx) = lens.find(['=', '-']) {
        (
            &lens[..idx],
            &lens[idx..idx + 1],
            lens[idx + 1..].parse::<u8>().unwrap_or(0),
        )
    } else {
        return;
    };

    let hash = hash(label.as_bytes()) as usize;

    match operation {
        "=" => {
            if !boxes[hash].iter().any(|(l, _)| l == &label) {
                boxes[hash].push((label, focal_length));
            } else {
                boxes[hash].iter_mut().for_each(|lens| {
                    if lens.0 == label {
                        lens.1 = focal_length;
                        return;
                    }
                });
            }
        }
        "-" => {
            if let Some(idx) = boxes[hash].iter().position(|(l, _)| l == &label) {
                boxes[hash].remove(idx);
            }
        }
        _ => unreachable!(),
    }
}

fn hash(text: &[u8]) -> u8 {
    text.iter()
        .fold(0, |acc, &c| acc.wrapping_add(c).wrapping_mul(17))
}
