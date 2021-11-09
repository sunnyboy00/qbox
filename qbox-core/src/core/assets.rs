use crate::broker::*;
use serde::{Deserialize, Serialize};

#[doc = "账户种类"]
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum Kind {
    Cash,    //现金
    Futures, //期货
    Options, //期权
    Fund,    //基金
    Bond,    //债券
    Stock,   //股票
    Swap,    //永续
}

#[doc = "账户"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    pub unit: String,
    pub account: String,
    pub kind: Kind,
    pub amount: f64, //量
    // pub frozen_amount: f64, //冻结量
    pub state: String,
}

//Ledger 账本定义
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Ledger<'a> {
    pub id: &'a str,
    pub unit: &'a str,
    pub account: &'a str,
    pub side: Side,
    pub open: f64,
    pub amount: f64,
    pub close: f64,
    pub uses: &'a str, //用途，买开[多]，卖平，卖开[空]，买平，冻结，解冻，手续费，结算，保证金
    pub bizid: &'a str, //业务ID
    pub bizop: &'a str, //业务操作代码，
    pub time: i64,
    pub remark: &'a str, //备注
}
