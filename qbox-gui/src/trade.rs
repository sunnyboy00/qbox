use crate::counter::Exchange;
use anyhow::Result;
use chrono::{DateTime, Utc};
use druid::{Data, Lens};
use serde::{Deserialize, Serialize};
use std::ops::BitOr;

//
// 做多：买入开仓，卖出平仓。(Side::Buy | Side::Open)=Side::Long ; Side::Sell | Side::Close;
// 做空：卖出开仓，买入平仓。( Side::Sell | Side::Open )= Side::Short; Side::Buy | Side::Close;
// 1、买开：买入开仓（做多）
// 2、买平：买入平仓（平掉持有的空单）
// 3、卖开：卖出开仓（做空）
// 4、卖平：卖出平仓（平掉持有的多单）

// 【买进开仓】：是指投资者对未来价格趋势看涨而采取的交易手段，买进持有看涨合约，意味着帐户资金买进合约而冻结。
// 【卖出平仓】：是指投资者对未来价格趋势不看好而采取的交易手段，而将原来买进的看涨合约卖出，投资者资金帐户解冻。

// 【卖出开仓】：是指投资者对未来价格趋势看跌而采取的交易手段，卖出看跌合约。卖出开仓，帐户资金冻结。
// 【买进平仓】：是指投资者将持有的卖出合约对未来行情不再看跌而补回以前卖出合约，与原来的卖出合约对冲抵消退出市场，帐户资金解冻
#[doc = "方向"]
#[derive(Debug, Data, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum Side {
    Buy,    //买
    Sell,   //卖
    Lock,   //锁仓
    Unlock, //解锁
    Exec,   //行权
    Drop,   //弃权
    Bid,    //出价
    Ask,    //要价
    Maker,  //被动成交，挂单
    Taker,  //主动成交，吃单
    Long,   //多
    Short,  //空
    // LongYD,         //昨天多
    // ShortYD,        //昨天空
    Open,           //	开
    Close,          //	平
    CloseToday,     //	平今
    CloseYesterday, //	平昨
    Credit,         //贷
    Debit,          //借
}

impl BitOr for Side {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        //买开做多
        if self == Side::Buy && rhs == Side::Open {
            return Side::Long;
        }
        //卖平仓，对应买开仓记录
        if self == Side::Sell
            && (rhs == Side::Close || rhs == Side::CloseToday || rhs == Side::CloseYesterday)
        {
            return Side::Close;
        }
        //卖开做空
        if self == Side::Sell && rhs == Side::Open {
            return Side::Short;
        }
        //买平仓，对应卖开仓记录
        if self == Side::Buy
            && (rhs == Side::Close || rhs == Side::CloseToday || rhs == Side::CloseYesterday)
        {
            return Side::Close;
        }
        panic!("不支持{:?}|{:?}", self, rhs)
    }
}

impl Into<String> for Side {
    fn into(self) -> String {
        match self {
            Side::Buy => "买",
            Side::Sell => "卖",
            Side::Lock => "锁仓",
            Side::Unlock => "解仓",
            Side::Exec => "行权",
            Side::Drop => "弃权",
            Side::Bid => "出价",
            Side::Ask => "要价",
            Side::Maker => "被动",
            Side::Taker => "主动",
            Side::Long => "多",
            Side::Short => "空",
            Side::Open => "开",
            Side::Close => "平",
            Side::CloseToday => "平今",
            Side::CloseYesterday => "平昨",
            Side::Credit => "贷",
            Side::Debit => "借",
        }
        .into()
    }
}

#[doc = "交易品种"]
#[derive(Debug, Data, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum TradeKind {
    SPOT,    //现货
    SWAP,    //永续
    OPTIONS, //期权
    FUTURES, //期货
    BOND,    //债券
    Unknown, //未知
}

impl ToString for TradeKind {
    fn to_string(&self) -> String {
        match self {
            TradeKind::SPOT => "SPOT",
            TradeKind::SWAP => "SWAP",
            //  TradeKind::SPOTMARGIN => "SPOTMARGIN",
            TradeKind::OPTIONS => "OPTIONS",
            TradeKind::FUTURES => "FUTURES",
            TradeKind::BOND => "BOND",
            TradeKind::Unknown => "unknown",
        }
        .into()
    }
}

impl Default for TradeKind {
    fn default() -> Self {
        TradeKind::Unknown
    }
}

impl From<&str> for TradeKind {
    fn from(s: &str) -> Self {
        let s = s.to_uppercase();
        match s.as_str() {
            "SPOT" => TradeKind::SPOT,
            "SWAP" => TradeKind::SWAP,
            // "MARGIN" => TradeKind::SPOTMARGIN,
            "OPTION" => TradeKind::OPTIONS,
            "FUTURE" => TradeKind::FUTURES,
            "BOND" => TradeKind::BOND,
            _ => TradeKind::Unknown,
        }
    }
}

impl<'a> Into<&'a str> for TradeKind {
    fn into(self) -> &'a str {
        match self {
            TradeKind::SPOT => "SPOT",
            TradeKind::SWAP => "SWAP",
            //  TradeKind::SPOTMARGIN => "SPOTMARGIN",
            TradeKind::OPTIONS => "OPTIONS",
            TradeKind::FUTURES => "FUTURES",
            TradeKind::BOND => "BOND",
            TradeKind::Unknown => "unknown",
        }
    }
}

impl Into<String> for TradeKind {
    fn into(self) -> String {
        match self {
            TradeKind::SPOT => "SPOT",
            TradeKind::SWAP => "SWAP",
            //  TradeKind::SPOTMARGIN => "SPOTMARGIN",
            TradeKind::OPTIONS => "OPTIONS",
            TradeKind::FUTURES => "FUTURES",
            TradeKind::BOND => "BOND",
            TradeKind::Unknown => "unknown",
        }
        .into()
    }
}

//Transaction 账户方成交明细
#[derive(Debug, Data, Copy, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub id: &'static str,
    pub exchange: &'static str,
    pub instrument: &'static str, //交易对，例如：BTC-USDT
    pub time: i64,
    pub account: &'static str, //交易主体的账户，USDT
    pub side: Side,            //方向，buy or sell
    //pub order_side: Side,      //订单方向，buy or sell
    //pub into_side: Side,       //成交方向，taker/maker（主动成交/被动成交）
    // pub offset: Offset,             //开平方向
    pub price: f64,                 //成交价格
    pub quantity: f64,              //成交数量
    pub fee: f64,                   //手续费
    pub ask_order_id: &'static str, //Ask订单号
    pub bid_order_id: &'static str, //Bid订单号
}

//仓位信息
#[derive(Debug, Data, Clone, Deserialize, Serialize)]
pub enum Position {
    //期货
    Futures {
        exchange: String,
        instrument: String,  //合约代码
        side: Side, // PD_LONG为多头仓位(CTP中用closebuy_today平仓),PD_SHORT为空头仓位(CTP用closesell_today)平仓,(CTP期货中)PD_LONG_YD为咋日多头仓位(用closebuy平),PD_SHORT_YD为咋日空头仓位(用closesell平)
        margin_level: u16, // 杆杠大小
        quantity: i64, // 持仓量，OKEX合约交易所，表示合约的份数(整数且大于1，即合约张数)
        frozen: f64, // 仓位冻结量
        last: f64,  //最新价
        average: f64, // 持仓均价
        settlement: f64, //结算价
        cost: f64,  //持仓成本
        margin: f64, // 仓位占用的保证金
        realized_pnl: f64, //已实现盈亏
        unrealized_pnl: f64, //未实现盈亏
        position_pnl: f64, // 持仓浮动盈亏(数据货币单位：BTC/LTC,传统期货单位:RMB,股票不支持此字段,注:OKEX合约全仓情况下指实现盈余,并非持仓盈亏,逐仓下指持仓盈亏)
    },
    //现货
    Spot {
        exchange: String,
        instrument: String,
        side: Side, // PD_LONG为多头仓位(CTP中用closebuy_today平仓),PD_SHORT为空头仓位(CTP用closesell_today)平仓,(CTP期货中)PD_LONG_YD为咋日多头仓位(用closebuy平),PD_SHORT_YD为咋日空头仓位(用closesell平)
        quantity: i64, // 持仓量，OKEX合约交易所，表示合约的份数(整数且大于1，即合约张数)
        margin_level: u16, // 杆杠大小
        last_price: f64, //最新价
        avg_price: f64, // 持仓均价
        close_price: f64, //收盘价
        cost_price: f64, //持仓成本
        realized_pnl: f64, //已实现盈亏
        unrealized_pnl: f64, //未实现盈亏
    },
}

//计价货币
#[derive(Default, Data, Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub summary: String,
    pub description: String,
}

impl Currency {
    pub fn new<T: Into<String>>(code: T) -> Self {
        Self {
            code: code.into(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Data, Clone, Deserialize, Serialize, PartialEq)]
pub enum MarginPriceType {
    Pre,
    Last,
}
#[doc = "证券"]
#[derive(Debug, Data, Lens, Clone, Deserialize, Serialize, PartialEq)]
pub struct Instrument {
    pub exchange: Exchange,
    pub symbol: String,      //证券名称
    pub security_id: String, //证券代码
    pub kind: TradeKind,
    pub base: String,                       //基础货币
    pub base_precision: f64,                //基础货币精度
    pub quote: String,                      //计价货币
    pub quote_precision: f64,               //计价货币精度
    pub tick_precision: [f64; 2],           //0为报价精度，1为数量精度
    pub margin_level: usize,                // 杆杠大小
    pub fee_margin: f64,                    //保证金费率
    pub fee_take: f64,                      //主动成交费率，成交一笔算一笔
    pub fee_make: f64,                      //被动成交费率，成交一笔算一笔
    pub fee_offer: f64,                     //委托/申报费率，包括报、撤单
    pub pre_settlement_price: f64,          //昨结算价
    pub last_settlement_price: f64,         //最新结算价
    pub average_price: f64,                 //成交均价
    pub open_price: f64,                    //开仓价
    pub margin_price_type: MarginPriceType, //保证金计算类型
    pub myself: bool,                       //是否自选
}
// impl Lens for Instrument {
//     fn with<V, F: FnOnce(&U) -> V>(&self, data: &T, f: F) -> V;

//     /// Get mutable access to the field.
//     ///
//     /// This method is defined in terms of a closure, rather than simply
//     /// yielding a mutable reference, because it is intended to be used
//     /// with value-type data (also known as immutable data structures).
//     /// For example, a lens for an immutable list might be implemented by
//     /// cloning the list, giving the closure mutable access to the clone,
//     /// then updating the reference after the closure returns.
//     fn with_mut<V, F: FnOnce(&mut U) -> V>(&self, data: &mut T, f: F) -> V;
// }
impl Instrument {
    pub fn with_exchange(mut self, ex: Exchange) -> Self {
        self.exchange = ex;
        self
    }
    pub fn with_symbol<T: Into<String>>(mut self, symbol: T) -> Self {
        self.symbol = symbol.into();
        self
    }
    pub fn with_secrity_id<T: Into<String>>(mut self, security_id: T) -> Self {
        self.security_id = security_id.into();
        self
    }
    pub fn with_kind(mut self, kind: TradeKind) -> Self {
        self.kind = kind;
        self
    }
    pub fn with_base<T: Into<String>>(mut self, base: T) -> Self {
        self.base = base.into();
        self
    }
    pub fn with_base_precision(mut self, precision: f64) -> Self {
        self.base_precision = precision;
        self
    }

    pub fn with_quote<T: Into<String>>(mut self, base: T) -> Self {
        self.base = base.into();
        self
    }
    pub fn with_quote_precision(mut self, precision: f64) -> Self {
        self.quote_precision = precision;
        self
    }

    pub fn with_tick_precision(mut self, tick_precision: (f64, f64)) -> Self {
        self.tick_precision = [tick_precision.0, tick_precision.1];
        self
    }

    pub fn with_margin_level(mut self, margin_level: usize) -> Self {
        self.margin_level = margin_level;
        self
    }

    pub fn with_fee_margin(mut self, val: f64) -> Self {
        self.fee_margin = val;
        self
    }
    pub fn with_fee_take(mut self, val: f64) -> Self {
        self.fee_take = val;
        self
    }
    pub fn with_fee_make(mut self, val: f64) -> Self {
        self.fee_make = val;
        self
    }
    pub fn with_fee_offer(mut self, val: f64) -> Self {
        self.fee_offer = val;
        self
    }
    pub fn with_pre_settlement_price(mut self, val: f64) -> Self {
        self.pre_settlement_price = val;
        self
    }
    pub fn with_last_settlement_price(mut self, val: f64) -> Self {
        self.fee_margin = val;
        self
    }
    pub fn with_average_price(mut self, val: f64) -> Self {
        self.average_price = val;
        self
    }
    pub fn with_open_price(mut self, val: f64) -> Self {
        self.open_price = val;
        self
    }
    pub fn with_margin_price_type(mut self, val: MarginPriceType) -> Self {
        self.margin_price_type = val;
        self
    }
    pub fn with_myself(mut self, val: bool) -> Self {
        self.myself = val;
        self
    }
}

impl Default for Instrument {
    fn default() -> Self {
        Instrument {
            exchange: Default::default(),
            symbol: "黄金2021".into(),
            security_id: "HU2021".into(),
            kind: TradeKind::FUTURES,
            base: "CNY".into(),
            base_precision: 1.0,
            quote: "CNY".into(),
            quote_precision: 1.0,
            tick_precision: [f64::NAN, f64::NAN],
            margin_level: 1,
            fee_margin: f64::NAN,
            fee_take: f64::NAN,
            fee_make: f64::NAN,
            fee_offer: f64::NAN,
            pre_settlement_price: f64::NAN,
            last_settlement_price: f64::NAN,
            average_price: f64::NAN,
            open_price: f64::NAN,
            margin_price_type: MarginPriceType::Pre,
            myself: false,
        }
    }
}

#[doc = "委托单状态"]
#[derive(Debug, Data, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct OrderState {
    pub filled_quantity: f64,
    pub filled_amount: f64,
    pub avg_price: f64,
    pub last_time: i64,
    pub state: State,
}

#[doc = "委托单有效期"]
#[derive(Debug, Data, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum OrderLife {
    GTC,         //Good-Till-Cancelled，一直有效至取消
    GIS,         //交易时间段有效
    GTD(String), //指定日前有效
    ROD,         //Rest of Day，同GFD，日内有效
    AON,         //All-Or-None，全部成交或零成交，不能取消
    IOC, //Immediate-Or-Cancel，同FAK，立即成交否則取消，能成交多少就成交多少，不能成交的就撤消
    FAK, //Fill-Any-Kill，同IOC，立即成交，不能成交部分撤销
    FOK, //Fill-Or-Kill，立即全部成交否則取消
    FAS, //Fill-Or-Save，部分成交其余保留
}

impl Default for OrderLife {
    fn default() -> Self {
        OrderLife::GTC
    }
}

#[doc = "委托单"]
#[derive(Debug, Data, Clone, Deserialize, Serialize, PartialEq)]
pub enum Order {
    //限价单
    Limit {
        id: String,
        exchange: Exchange,
        instrument: Instrument,
        time: i64,
        trade_kind: TradeKind, // 交易品种
        side: Side,
        //offset: Option<Offset>,
        price: f64,
        quantity: f64,
        lever: u16,
        pov: OrderLife, //period of validity
        remark: String, //备注
        state: OrderState,
    },
    //市价单
    Market {
        id: String,
        exchange: Exchange,
        instrument: Instrument,
        time: i64,
        trade_kind: TradeKind, // 交易品种
        side: Side,
        //PositionEffect 开平标志
        //offset: Option<Offset>,
        quantity: f64,
        lever: u16,
        pov: OrderLife,
        state: OrderState,
    },
    //止盈止损单
    TakeStop {
        id: String,
        exchange: Exchange,
        instrument: Instrument,
        time: i64,
        trade_kind: TradeKind, // 交易品种
        side: Side,
        //offset: Option<Offset>,
        price: f64,
        quantity: f64,
        lever: u16,
        trigger_price: f64, //触发价格
        pov: OrderLife,
        state: OrderState,
    },
    //跟踪委托单
    Tracking {
        id: String,
        exchange: Exchange,
        instrument: Instrument,
        time: i64,
        trade_kind: TradeKind, // 交易品种
        side: Side,
        //offset: Option<Offset>,
        price: f64,
        quantity: f64,
        lever: u16,
        callback_rate: f64, //回调幅度，填写值0.001（0.1%）\<=X\<=0.05（5%）
        trigger_price: f64, //激活价格 ，填写值0\<X\<=1000000
        pov: OrderLife,
        state: OrderState,
    },
    //冰山委托
    Iceberg {
        id: String,
        exchange: Exchange,
        instrument: Instrument,
        time: i64,
        trade_kind: TradeKind, // 交易品种
        side: Side,
        //offset: Option<Offset>,
        price: f64,
        quantity: f64,
        lever: u16,
        variance: f64,    //委托深度，填写值0.0001(0.01%)\<=X\<=0.01（1%）
        avg_amount: f64,  //单笔均值，填写2-1000的整数（永续2-500的整数）
        limit_price: f64, //价格限制 ，填写值0\<X\<=1000000
        pov: OrderLife,
        state: OrderState,
    },
    //时间加权
    TimeWeights {
        id: String,
        exchange: Exchange,
        instrument: Instrument,
        time: i64,
        trade_kind: TradeKind, // 交易品种
        side: Side,
        //offset: Option<Offset>,
        price: f64,
        quantity: f64,
        lever: u16,
        sweep_range: f64,   //扫单范围，填写值0.005（0.5%）\<=X\<=0.01（1%）
        sweep_ratio: f64,   //扫单比例，填写值 0.01\<=X\<=1
        single_limit: f64,  //单笔上限，填写值10\<=X\<=2000（永续2-500的整数）
        limit_price: f64,   //价格限制，填写值0\<X\<=1000000
        time_interval: f64, //委托间隔，填写值5\<=X\<=120
        pov: OrderLife,
        state: OrderState,
    },
}

#[derive(Debug, Data, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum State {
    Created,             //创建，初始状态
    Submitted,           //	已提交
    Accepted,            //已受理
    Rejected,            //拒绝
    Cancelled,           //	已撤单
    Expired,             //过期
    Filled,              //	已成交
    PartFilledNotActive, //	部分成交不在队列中（部成部撤）
    PartFilledActive,    //	部分成交还在队列中
}
