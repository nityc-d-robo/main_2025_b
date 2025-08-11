use std::sync::{Arc, RwLock};

pub struct RetainingArmState {
    pub test: usize,
}

impl RetainingArmState {
    pub fn new() -> Self {
        RetainingArmState { test: 0 }
    }
}

// entryはArcでラップしたRwLockを受け取る設計にする
pub fn entry(r_a_state: Arc<RwLock<RetainingArmState>>) {
    loop {
        // 参照のみ使いたいときはreadロックを取得
        let state = r_a_state.read().unwrap();
        if state.test == 0 {}
    }
}
