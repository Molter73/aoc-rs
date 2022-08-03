use std::thread;

pub struct AdventCoins {
    seed: String,
    target: String,
}

impl AdventCoins {
    pub fn new(seed: &str, target: &str) -> AdventCoins {
        AdventCoins {
            seed: seed.to_string(),
            target: target.to_string(),
        }
    }

    pub fn mine(&self) -> u64 {
        let mut lower: u64 = 0;
        let offset: u64 = 10000;

        loop {
            let mut threads = Vec::new();

            for _ in 0..10 {
                let seed = self.seed.clone();
                let target = self.target.clone();

                let handle =
                    thread::spawn(move || AdventCoins::worker(seed, target, lower, lower + offset));

                threads.push(handle);
                lower += offset;
            }

            for result in threads {
                let result = result.join().unwrap();
                if let Some(r) = result {
                    return r;
                }
            }
        }
    }

    fn worker(seed: String, target: String, start: u64, stop: u64) -> Option<u64> {
        for lower in start..stop {
            let input: String = seed.clone() + &lower.to_string();
            let hash = md5::compute(input);
            let digest = format!("{:x}", hash);

            if digest.starts_with(&target) {
                return Some(lower);
            }
        }
        None
    }
}
