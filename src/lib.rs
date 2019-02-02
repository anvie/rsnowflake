use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Copy, Clone)]
pub struct SnowflakeIdGenerator {
    machine_id: u32,
    idx: u64,
}

impl SnowflakeIdGenerator {
    pub fn new(machine_id: u32) -> SnowflakeIdGenerator {
        SnowflakeIdGenerator {
            machine_id: machine_id,
            idx: 0,
        }
    }

    pub fn generate(&mut self) -> i64 {
        self.idx = self.idx + 1;

        // a SnowFlake style
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went mackward")
            .as_millis();

        (millis << 22) as i64 | ((self.machine_id << 12) as i64) | (self.idx as i64)
    }
}

#[cfg(test)]
mod test {

    use super::SnowflakeIdGenerator;

    #[test]
    fn test_generate() {
        let mut idgen = SnowflakeIdGenerator::new(1);
        let mut ids = vec![];

        for _ in 0..10 {
            ids.push(idgen.generate());
        }

        for id in ids {
            println!("id: {}", id);
            assert!(format!("{}", id).len() >= 18);
        }
    }
}
