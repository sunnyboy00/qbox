use crate::{Item, Parameter, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::BitOr;
use ta::{Close, High, Low, Open, Volume};

#[doc = "交易事件"]
#[derive(Debug)]
pub enum TradeEvent {
    Offer(Order),
    OfferResponse(Order),
    Cancel(Order),
    CancelResponse(Order),
    //QueryPosition(String),
    PositionResponse(Position),
    // QueryInstrument(Vec<String>),
    InstrumentsResponse(Instrument),
    TransactionNotify(Transaction),
}

#[doc = "行情"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum QuoteEvent {
    //逐笔委托
    TickToOffer(TickToOffer),
    //逐笔成交/快照
    TickToTrade(TickToTrade),
    //基本行情
    Level1(Level1),
    //深度行情
    Level2(Level2),
    //k线
    Bar(Bar),
}

impl ToString for QuoteEvent {
    fn to_string(&self) -> String {
        if let Ok(s) = serde_json::to_string(&self) {
            s
        } else {
            "".to_string()
        }
    }
}

#[doc = "交易所"]
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Exchange {
    SSE,       //上交所
    SZE,       //深交所
    SHFE,      //上期所
    DCE,       //大商所
    DZCE,      //郑商所
    CFFEX,     //中金所
    INE,       //能源中心
    OKEX,      //OKEX
    BINANCE,   //BINANCE
    HUOBI,     //HUOBI
    KRX,       //韩国交易所
    NSE,       //印度国家交易所
    EUREX,     //欧洲期货交易所(EUREX)
    CBOE,      //芝加哥期权交易所(CBOE)
    TAIFEX,    //台湾期货交易所(TAIFEX)
    TASE,      //以色列特拉维夫证交所(TASE)
    CME,       //芝加哥商业交易所(CME)
    OSAKE,     //大阪交易所(OSAKE)
    NYSELIFFE, //泛欧交易所(NYSE.LIFFE)
    HKFX,      //香港交易所（HKFX）
    UNKNOWN,
}

impl Default for Exchange {
    fn default() -> Self {
        Self::UNKNOWN
    }
}

impl<S: AsRef<str>> From<S> for Exchange {
    fn from(s: S) -> Self {
        let s = s.as_ref().to_uppercase();
        match s.as_str() {
            "SSE" => Exchange::SSE,
            "SZE" => Exchange::SZE,
            "SHFE" => Exchange::SHFE,
            "DCE" => Exchange::DCE,
            "DZCE" => Exchange::DZCE,
            "CFFEX" => Exchange::CFFEX,
            "INE" => Exchange::INE,
            "OKEX" => Exchange::OKEX,
            "BINANCE" => Exchange::BINANCE,
            "HUOBI" => Exchange::HUOBI,
            "KRX" => Exchange::KRX,
            "NSE" => Exchange::NSE,
            "EUREX" => Exchange::EUREX,
            "CBOE" => Exchange::CBOE,
            "TAIFEX" => Exchange::TAIFEX,
            "TASE" => Exchange::TASE,
            "CME" => Exchange::CME,
            "OSAKE" => Exchange::OSAKE,
            "NYSE.LIFFE" => Exchange::NYSELIFFE,
            "HKFX" => Exchange::HKFX,
            _ => Exchange::UNKNOWN,
        }
    }
}

impl<'a> Into<&'a str> for Exchange {
    fn into(self) -> &'a str {
        match self {
            Exchange::SSE => "SSE",
            Exchange::SZE => "SZE",
            Exchange::SHFE => "SHFE",
            Exchange::DCE => "DCE",
            Exchange::DZCE => "DZCE",
            Exchange::CFFEX => "CFFEX",
            Exchange::INE => "INE",
            Exchange::OKEX => "OKEX",
            Exchange::BINANCE => "BINANCE",
            Exchange::HUOBI => "HUOBI",
            Exchange::KRX => "KRX",
            Exchange::NSE => "NSE",
            Exchange::EUREX => "EUREX",
            Exchange::CBOE => "CBOE",
            Exchange::TAIFEX => "TAIFEX",
            Exchange::TASE => "TASE",
            Exchange::CME => "CME",
            Exchange::OSAKE => "OSAKE",
            Exchange::NYSELIFFE => "NYSE.LIFFE",
            Exchange::HKFX => "HKFX",
            Exchange::UNKNOWN => "UNKNOWN",
        }
    }
}

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
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum Side {
    Buy,            //买
    Sell,           //卖
    Lock,           //锁仓
    Unlock,         //解锁
    Exec,           //行权
    Drop,           //弃权
    Bid,            //出价
    Ask,            //要价
    Maker,          //被动成交，挂单
    Taker,          //主动成交，吃单
    Long,           //多
    Short,          //空
    Call,           //看涨
    Put,            //看跌
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
            Side::Call => "看涨",
            Side::Put => "看跌",
        }
        .into()
    }
}

#[doc = "交易品种"]
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum TradeKind {
    SPOT,    //现货
    SWAP,    //永续
    OPTIONS, //期权
    FUTURES, //期货
    BOND,    //债券
    Unknown, //未知
}

impl Default for TradeKind {
    fn default() -> Self {
        TradeKind::Unknown
    }
}

impl<S: AsRef<str>> From<S> for TradeKind {
    fn from(s: S) -> Self {
        let s = s.as_ref().to_uppercase();
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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub id: u64,
    pub order_id: u64,
    pub out_id: String,
    pub exchange: Exchange,
    pub security_id: String, //证券代码
    pub time: i64,
    // pub account: String, //交易主体的账户，USDT
    pub side: Side, //成交方向
    //pub order_side: Side,      //订单方向，buy or sell
    pub into_side: Side, //成交方向，taker/maker（主动成交/被动成交）
    // pub offset: Side,                 //开平方向
    pub price: f64,    //成交价格
    pub quantity: f64, //成交数量
    // pub fee: f64,                     //手续费
    pub ask_order_id: Option<String>, //Ask订单号
    pub bid_order_id: Option<String>, //Bid订单号
}

//仓位信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Position {
    //期货
    pub exchange: Exchange,
    pub security_id: String, //证券代码
    pub side: Side, // PD_LONG为多头仓位(CTP中用closebuy_today平仓),PD_SHORT为空头仓位(CTP用closesell_today)平仓,(CTP期货中)PD_LONG_YD为咋日多头仓位(用closebuy平),PD_SHORT_YD为咋日空头仓位(用closesell平)
    pub offset: Side,
    pub margin_level: u16,   // 杆杠大小
    pub quantity: i64,       // 持仓量，OKEX合约交易所，表示合约的份数(整数且大于1，即合约张数)
    pub frozen: f64,         // 仓位冻结量
    pub last: f64,           //最新价
    pub average: f64,        // 持仓均价
    pub settlement: f64,     //结算价
    pub cost: f64,           //持仓成本
    pub margin: f64,         // 仓位占用的保证金
    pub realized_pnl: f64,   //已实现盈亏
    pub unrealized_pnl: f64, //未实现盈亏
    pub position_pnl: f64, // 持仓浮动盈亏(数据货币单位：BTC/LTC,传统期货单位:RMB,股票不支持此字段,注:OKEX合约全仓情况下指实现盈余,并非持仓盈亏,逐仓下指持仓盈亏)
                           // //现货
                           // Spot {
                           //     exchange: Exchange,
                           //     security_id: String, //证券代码
                           //     side: Side, // PD_LONG为多头仓位(CTP中用closebuy_today平仓),PD_SHORT为空头仓位(CTP用closesell_today)平仓,(CTP期货中)PD_LONG_YD为咋日多头仓位(用closebuy平),PD_SHORT_YD为咋日空头仓位(用closesell平)
                           //     offset: Side,
                           //     quantity: i64, // 持仓量，OKEX合约交易所，表示合约的份数(整数且大于1，即合约张数)
                           //     margin_level: u16, // 杆杠大小
                           //     last: f64,     //最新价
                           //     average: f64,  // 持仓均价
                           //     close: f64,    //收盘价
                           //     cost: f64,     //持仓成本
                           //     realized_pnl: f64, //已实现盈亏
                           //     unrealized_pnl: f64, //未实现盈亏
                           // },
}

//计价货币
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub precision: f64, //精度
}

impl Currency {
    pub fn new<T: Into<String>>(code: T) -> Self {
        Self {
            code: code.into(),
            ..Default::default()
        }
    }
    pub fn with_name<T: Into<String>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }
    pub fn with_precision(mut self, val: f64) -> Self {
        self.precision = val;
        self
    }
}

impl Default for Currency {
    fn default() -> Self {
        Currency::new("CNY").with_name("人民币")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum MarginPriceType {
    Pre,
    Last,
}
#[doc = "证券状态"]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum InstState {
    NotStart, //未上市
    Started,  //上市
    Pause,    //停牌
    Trading,  //交易中
    Expired,  //到期
    Unknown,  //未知
}

impl<S: AsRef<str>> From<S> for InstState {
    fn from(s: S) -> Self {
        let s = s.as_ref();
        match s {
            "NotStart" => InstState::NotStart,
            "Started" => InstState::Started,
            "Pause" => InstState::Pause,
            "Trading" => InstState::Trading,
            "Expired" => InstState::Expired,
            _ => InstState::Unknown,
        }
    }
}

impl<'a> Into<&'a str> for InstState {
    fn into(self) -> &'a str {
        match self {
            InstState::NotStart => "NotStart",
            InstState::Started => "Started",
            InstState::Pause => "Pause",
            InstState::Trading => "Trading",
            InstState::Expired => "Expired",
            InstState::Unknown => "Unknown",
        }
    }
}

impl Default for InstState {
    fn default() -> Self {
        InstState::Unknown
    }
}

#[doc = "证券"]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Instrument {
    pub security_id: String, //证券代码
    pub exchange: Exchange,
    pub symbol: String, //证券名称
    pub kind: TradeKind,
    pub base_currency: String,  //基础货币
    pub quote_currency: String, //计价货币
    pub items: Parameter,
    pub multiplier: usize, //乘数
    pub state: InstState,
}

impl Instrument {
    pub fn new() -> Self {
        Self {
            exchange: Exchange::UNKNOWN,
            symbol: "".into(),
            security_id: "".into(),
            kind: TradeKind::FUTURES,
            base_currency: "".into(),
            quote_currency: "".into(),
            multiplier: 1,
            items: Parameter::with_capacity(100),
            state: InstState::Unknown,
        }
    }
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

    pub fn with_base_currency(mut self, val: String) -> Self {
        self.base_currency = val;
        self
    }

    pub fn with_quote_currency(mut self, val: String) -> Self {
        self.quote_currency = val;
        self
    }
    pub fn with_items(mut self, val: Parameter) -> Self {
        self.items = val;
        self
    }
    pub fn with_item<I: Into<Item>>(mut self, item: I, val: Value) -> Self {
        self.items.insert(item.into(), val);
        self
    }

    pub fn with_state(mut self, state: InstState) -> Self {
        self.state = state;
        self
    }

    pub fn with_multiplier(mut self, val: usize) -> Self {
        self.multiplier = val;
        self
    }
}

impl Default for Instrument {
    fn default() -> Self {
        Instrument::new()
    }
}

#[doc = "委托单状态"]
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct OrderState {
    pub filled_quantity: f64,
    pub filled_amount: f64,
    pub avg_price: f64,
    pub last_time: i64,
    pub state: State,
}

#[doc = "委托单有效期"]
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
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
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Order {
    //限价单
    Limit {
        id: u64,
        security_id: String, //证券代码
        exchange: Exchange,
        time: i64,
        side: Side,
        offset: Side,
        price: f64,
        quantity: f64,
        lever: u16,
        pov: OrderLife, //period of validity
        remark: String, //备注
        state: OrderState,
    },
    //市价单
    Market {
        id: u64,
        security_id: String, //证券代码
        exchange: Exchange,
        time: i64,
        side: Side,
        //开平标志
        offset: Side,
        quantity: f64,
        lever: u16,
        pov: OrderLife,
        state: OrderState,
    },
    //止盈止损单
    TakeStop {
        id: u64,
        security_id: String, //证券代码
        exchange: Exchange,
        time: i64,
        side: Side,
        offset: Side,
        price: f64,
        quantity: f64,
        lever: u16,
        trigger_price: f64, //触发价格
        pov: OrderLife,
        state: OrderState,
    },
    //跟踪委托单
    Tracking {
        id: u64,
        security_id: String, //证券代码
        exchange: Exchange,
        time: i64,
        side: Side,
        offset: Side,
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
        id: u64,
        security_id: String, //证券代码
        exchange: Exchange,
        time: i64,
        side: Side,
        offset: Side,
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
        id: u64,
        security_id: String, //证券代码
        exchange: Exchange,
        time: i64,
        side: Side,
        offset: Side,
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

impl Order {
    pub fn security_id(&self) -> &str {
        match self {
            Order::Limit { security_id, .. } => security_id.as_ref(),
            Order::Market { security_id, .. } => security_id.as_ref(),
            Order::TakeStop { security_id, .. } => security_id.as_ref(),
            Order::Tracking { security_id, .. } => security_id.as_ref(),
            Order::Iceberg { security_id, .. } => security_id.as_ref(),
            Order::TimeWeights { security_id, .. } => security_id.as_ref(),
        }
    }
    pub fn exchange(&self) -> Exchange {
        match self {
            &Order::Limit { exchange, .. } => exchange,
            &Order::Market { exchange, .. } => exchange,
            &Order::TakeStop { exchange, .. } => exchange,
            &Order::Tracking { exchange, .. } => exchange,
            &Order::Iceberg { exchange, .. } => exchange,
            &Order::TimeWeights { exchange, .. } => exchange,
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
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

#[doc = "行情深度，价、量、委托笔数、委托额"]
pub type Depth = (f64, f64, f64, f64); //[价,量,委托数,委托额]

#[doc = "逐笔委托"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TickToOffer {
    pub security_id: String, //证券代码
    pub exchange: Exchange,
    pub time: i64,
    pub side: Side,
    pub price: f64,
    pub quantity: f64,
    pub bids: Option<Vec<Depth>>,
    pub asks: Option<Vec<Depth>>,
}

#[doc = "逐笔成交"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TickToTrade {
    pub security_id: String, //证券代码
    pub exchange: Exchange,
    pub id: String,
    pub time: i64,                     //时间
    pub price: f64,                    //价
    pub quantity: f64,                 //量
    pub order_side: Option<Side>,      //订单方向
    pub into_side: Option<Side>,       //主动（taker）成交方向
    pub take_order_id: Option<String>, //买单ID
    pub make_order_id: Option<String>, //卖单ID
}

#[doc = "基本行情"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Level1 {
    pub security_id: String, //证券代码
    pub exchange: Exchange,
    pub trading_date: String,
    pub action_date: String,
    pub time: i64,
    pub open: f64,          //开盘价
    pub high: f64,          //最高价
    pub low: f64,           //最低价
    pub close: f64,         //收盘价
    pub bids: Vec<Depth>,   //出价
    pub asks: Vec<Depth>,   //要价
    pub average: f64,       //均价
    pub last: f64,          //最新价
    pub last_quantity: f64, //最新成交量
    pub volume: f64,        //24小时成交量
    pub turnover: f64,      //24小时最新成交额
    pub score: f64,         //得分
}

impl Level1 {
    pub fn to_bar(&self) -> Bar {
        Bar {
            security_id: self.security_id.clone(),
            exchange: self.exchange,
            time: self.time,
            open: self.open,
            high: self.high,
            low: self.low,
            close: if self.close == f64::NAN {
                self.last
            } else {
                self.close
            },
            volume: self.last_quantity,
            turnover: Some(self.turnover),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Bar {
    pub security_id: String, //证券代码
    pub exchange: Exchange,
    pub time: i64,
    pub open: f64,             //开盘价
    pub high: f64,             //最高价
    pub low: f64,              //最低价
    pub close: f64,            //收盘价
    pub volume: f64,           //成交量
    pub turnover: Option<f64>, //成交额
}

impl Open for Bar {
    #[inline]
    fn open(&self) -> f64 {
        self.open
    }
}

impl Close for Bar {
    #[inline]
    fn close(&self) -> f64 {
        self.close
    }
}

impl Low for Bar {
    #[inline]
    fn low(&self) -> f64 {
        self.low
    }
}

impl High for Bar {
    #[inline]
    fn high(&self) -> f64 {
        self.high
    }
}

impl Volume for Bar {
    #[inline]
    fn volume(&self) -> f64 {
        self.volume
    }
}

#[doc = "深度行情"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Level2 {
    pub security_id: String, //证券代码
    pub exchange: Exchange,
    pub time: i64, //时间
    pub bids: Vec<Depth>,
    pub asks: Vec<Depth>,
    #[serde(skip)]
    pub raw: Option<Vec<u8>>,
}

#[doc = "订单簿"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopOfOrderBook {
    pub security_id: String, //证券代码
    pub exchange: Exchange,
    pub bids: Vec<Depth>,
    pub asks: Vec<Depth>,
}
