use std::fs::File;
use std::io::prelude::*;
use rand::{thread_rng, Rng};

fn main() {
    let mut file = File::create("savage_tiger.txt").unwrap();
    let mut rng = thread_rng();
    for _ in 0..100 {
        let mut buffer = [0; 256];
        let mut size = buffer.len();
        while size > 0 {
            let n = rng.gen_range(0, size);
            let n = std::cmp::min(n, size);
            file.write_all(&buffer[..n]).unwrap();
            size -= n;
        }
    }
}
