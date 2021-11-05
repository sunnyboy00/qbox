BEGIN;

CREATE TABLE IF NOT EXISTS qbox (
    unit TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (unit,key)
);

CREATE TABLE IF NOT EXISTS units (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    passwd TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS daemos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pid INTEGER,
    unit TEXT NOT NULL,
    kind TEXT NOT NULL, 
    name TEXT NOT NULL,
    uri TEXT NOT NULL,
    state TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS brokers (
    name TEXT NOT NULL PRIMARY KEY, 
    unit TEXT NOT NULL,
    items TEXT NOT NULL,
    remark TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS unit_instruments (
    unit TEXT NOT NULL,
    tag TEXT NOT NULL, 
    security_id TEXT NOT NULL,
    quota REAL,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (unit,tag,security_id)
);

CREATE TABLE IF NOT EXISTS instruments (
    security_id TEXT PRIMARY KEY,
    exchange TEXT,
    symbol TEXT,
    kind TEXT NOT NULL,
    base_currency TEXT NOT NULL,
    quote_currency TEXT NOT NULL,
    multiplier INTEGER NOT NULL DEFAULT 1,
    state TEXT NOT NULL,
    items TEXT,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS quote_level1 (
    security_id TEXT PRIMARY KEY,
    exchange TEXT,
    time INTEGER NOT NULL,
    avg REAL,
    open REAL,
    high REAL,
    low REAL,
    close REAL,
    last REAL,
    last_volum REAL,
    ask1 REAL,
    bid1 REAL,
    ask1_volume REAL,
    bid1_volume REAL,
    volume REAL,
    turnover REAL,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS quote_level2 (
    security_id TEXT PRIMARY KEY,
    exchange TEXT,
    time INTEGER NOT NULL,
    asks TEXT NOT NULL,
    bids TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS orders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    out_id TEXT UNIQUE,
    unit TEXT NOT NULL,
    security_id TEXT NOT NULL,
    exchange TEXT NOT NULL,
    kind TEXT NOT NULL,
    side TEXT NOT NULL,
    offset TEXT NOT NULL,
    price REAL,
    quantity REAL,
    multiplier INTEGER DEFAULT 1,
    pov TEXT NOT NULL DEFAULT 'GTC',
    filled_quantity REAL,
    filled_amount REAL,
    avg_price REAL,
    last_filled_time INTEGER NOT NULL,
    items TEXT,
    remark TEXT,
    owner TEXT NOT NULL,
    strategy TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'Created',
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS security_id ON orders (security_id);
CREATE INDEX IF NOT EXISTS owner ON orders (owner);

CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id TEXT NOT NULL,
    out_id TEXT,
    security_id TEXT NOT NULL,
    exchange TEXT NOT NULL,
    side TEXT NOT NULL,
    into_side TEXT,
    price REAL,
    quantity REAL,
    ask_order_id TEXT,
    bid_order_id TEXT,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS out_id_order_id ON transactions (out_id,order_id);
CREATE INDEX IF NOT EXISTS security_id_order_id ON transactions (security_id,order_id);

CREATE TABLE IF NOT EXISTS positions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pid INTEGER,
    unit TEXT NOT NULL,
    order_idINTEGER,
    security_id TEXT NOT NULL UNIQUE,
    exchange TEXT NOT NULL,
    side TEXT NOT NULL,
    offset TEXT NOT NULL,
    quantity REAL,
    frozen REAL,
    last REAL,
    average REAL,
    settlement  REAL,
    cost REAL,
    margin REAL,
    realized_pnl REAL,
    unrealized_pnl REAL,
    position_pnl REAL,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    unit TEXT NOT NULL,
    account TEXT NOT NULL ,
    amount REAL,
    associate INTEGER,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(unit, account)
);

CREATE TABLE IF NOT EXISTS ledgers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    unit TEXT NOT NULL,
    side TEXT NOT NULL,
    opcode TEXT NOT NULL,
    amount REAL,
    use TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMIT;