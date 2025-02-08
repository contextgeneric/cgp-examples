use std::collections::BTreeMap;

use cgp::prelude::*;

use crate::types::DemoCurrency;

pub struct MockApp {
    pub user_balances: BTreeMap<(String, DemoCurrency), u64>,
    pub user_passwords: BTreeMap<String, String>,
}

pub struct MockAppComponents;

impl HasComponents for MockApp {
    type Components = MockAppComponents;
}
