use num_bigint::BigInt;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("./challenge_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut numbers = VecDeque::with_capacity(100);
    let mut line_iter = reader.lines();

    for _ in 0..100 {
        if let Some(line) = line_iter.next() {
            let line = line?;
            if let Ok(n) = line.trim().parse::<BigInt>() {
                numbers.push_back(n);
            } else {
                println!("Invalid number in file: {}", line);
                return Ok(());
            }
        } else {
            println!("File has less than 100 lines.");
            return Ok(());
        }
    }

    let mut sums = HashSet::new();
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            sums.insert(&numbers[i] + &numbers[j]);
        }
    }

    let mut line_number = 100;
    let mut writer = BufWriter::new(io::stdout());
    for line in line_iter {
        line_number += 1;
        let line = line?;
        if let Ok(n) = line.trim().parse::<BigInt>() {
            if !sums.contains(&n) {
                writeln!(writer, "Number at line {} is not safe: {}", line_number, n)?;
            }
            update_sums(&mut sums, &mut numbers, &n);
        } else {
            writeln!(
                writer,
                "Invalid number in file at line {}: {}",
                line_number, line
            )?;
        }
    }
    Ok(())
}

fn update_sums(sums: &mut HashSet<BigInt>, numbers: &mut VecDeque<BigInt>, new_number: &BigInt) {
    if let Some(removed_number) = numbers.pop_front() {
        for num in &*numbers {
            sums.remove(&(removed_number.clone() + num));
            sums.insert(new_number.clone() + num);
        }
        numbers.push_back(new_number.clone());
    }
}

/*
-> Tried this method to have less .clone() but found out with cargo build --release then cargo bench that
its not faster :        time:   [82.210 ms 83.216 ms 84.067 ms]
                        change: [+4.7231% +6.0156% +7.4611%] (p = 0.00 < 0.05)
                        Performance has regressed.
so I'm staying with the previous implementation where I had:
                        time:   [75.976 ms 77.054 ms 77.745 ms]
                        change: [-9.9917% -8.1406% -6.6047%] (p = 0.00 < 0.05)
                        Performance has improved.

fn update_sums(sums: &mut HashSet<BigInt>, numbers: &mut VecDeque<BigInt>, new_number: &BigInt) {
    if let Some(removed_number) = numbers.pop_front() {
        for num in &*numbers {
            let sum_to_remove = &(&removed_number + num);
            sums.remove(sum_to_remove);

            let new_sum = new_number + num;
            sums.insert(new_sum);
        }
        numbers.push_back(new_number.clone());
    }
}
*/
