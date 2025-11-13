use std::collections::HashMap;
use std::sync::Mutex;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use bytes::Bytes;


pub type ShardedMutex = Mutex<HashMap<String, Bytes>>;    

pub struct ShardedDb {
    shard_count: usize,
    shards: Vec<ShardedMutex>,
}

impl ShardedDb {
    pub fn new(shard_count: usize) -> Self {
        let mut shards = Vec::with_capacity(shard_count);
        for _ in 0..shard_count {
            shards.push(Mutex::new(HashMap::new()));
        }
        Self { shard_count, shards }
    }

    pub fn get_shard(&self, key: &str) -> &ShardedMutex {
        let hash = calculate_hash(key) as usize;
        let shard_index = hash % self.shard_count;
        &self.shards[shard_index]
    }
}


fn calculate_hash<T: Hash + ?Sized>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}
