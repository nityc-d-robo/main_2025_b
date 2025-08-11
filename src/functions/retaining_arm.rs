use safe_drive::{logger::Logger, pr_info};
use std::sync::{Arc, RwLock};

use crate::NODE_NAME;

const FUNCTION_NAME: &str = "retaining_arm";

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
    let _logger = Logger::new(&format!("{}/{}", NODE_NAME, FUNCTION_NAME));
    loop {
        // 参照のみ使いたいときはreadロックを取得
        let state = r_a_state.read().unwrap();
        if state.test == 1 {
            pr_info!(_logger, "on");
        }
    }
}
