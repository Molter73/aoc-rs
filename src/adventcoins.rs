pub struct AdventCoins {
    seed: String
}

impl AdventCoins {
    pub fn new(seed: &str) -> AdventCoins {
        AdventCoins {
            seed: seed.to_string()
        }
    }

    pub fn mine(&self) -> u64 {
        let mut lower: u64 = 0;

        loop {
            let input: String = self.seed.clone() + &lower.to_string();
            let hash = md5::compute(input);
            let digest = format!("{:x}", hash);

            if digest.starts_with("00000") {
                return lower;
            }
            lower += 1;
        }
    }
}
