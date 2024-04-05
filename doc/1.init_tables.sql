create table arbitrage.arb_diff_rate
(
    id                  bigint auto_increment comment 'id'
        primary key,
    platform            varchar(64) default '' not null comment '平台 binance、okx、gate',
    coin                varchar(64)            not null comment '币种 BTC、ETH、DOT',
    option_choose       varchar(64)            not null comment 'positive, reverse',
    from_market         varchar(64)            not null comment 'From操作市场 spot、futures、delivery',
    from_symbol         varchar(64)            not null comment 'From操作交易对 现货 BTCUSDT、币本位 BTCUSD_210625、 U本位 BTCUSDT_210625',
    to_market           varchar(64)            not null comment 'To操作市场 spot、futures、delivery',
    to_symbol           varchar(64)            not null comment 'From操作交易对 现货 BTCUSDT、币本位 BTCUSD_210625、 U本位 BTCUSDT_210625',
    investment_currency varchar(64)            not null comment '投资币种 USDT、BTC',
    return_currency     varchar(64)            not null comment '回报币种 USDT、BTC',
    diff_status         tinyint     default 0  null comment '执行状态 0、不执行 1、执行',
    created             int         default 0  null comment '创建时间',
    updated             int         default 0  null comment '更新时间',
    bak                 varchar(255)           null comment '备注'
)
    comment '差价比率配置表' charset = utf8;

create table arbitrage.arb_diff_rate_his
(
    id           bigint auto_increment comment 'id'
        primary key,
    diff_rate_id bigint                        not null comment 'arb_diff_rate 表id',
    diff_price   decimal(20, 4) default 0.0000 not null comment '差价',
    diff_rate    decimal(20, 4) default 0.0000 not null comment '差价比率',
    created      int            default 0      null comment '创建时间',
    updated      int            default 0      null comment '更新时间',
    bak          varchar(255)                  null comment '备注'
)
    comment '差价比率数据历史记录表' charset = utf8mb4;

create table arbitrage.arb_diff_rate_info
(
    id            bigint auto_increment comment 'id'
        primary key,
    diff_rate_id  bigint                        not null comment 'arb_diff_rate 表id',
    platform      varchar(64)    default ''     not null comment '平台 binance、okx、gate',
    coin          varchar(64)                   not null comment '币种 BTC、ETH、DOT',
    option_choose varchar(64)                   not null comment 'positive, reverse',
    from_market   varchar(64)                   not null comment 'From操作市场 spot、futures、delivery',
    from_symbol   varchar(64)                   not null comment 'From操作交易对 现货 BTCUSDT、币本位 BTCUSD_210625、 U本位 BTCUSDT_210625',
    from_price    decimal(20, 4)                not null comment 'From价格',
    to_market     varchar(64)                   not null comment 'To操作市场 spot、futures、delivery',
    to_symbol     varchar(64)                   not null comment 'From操作交易对 现货 BTCUSDT、币本位 BTCUSD_210625、 U本位 BTCUSDT_210625',
    to_price      decimal(20, 4)                not null comment 'To价格',
    diff_price    decimal(20, 4) default 0.0000 not null comment '差价',
    diff_rate     decimal(20, 4) default 0.0000 not null comment '差价比率',
    created       int            default 0      null comment '创建时间',
    updated       int            default 0      null comment '更新时间',
    bak           varchar(255)                  null comment '备注'
)
    comment '差价数据表' charset = utf8;

create table arbitrage.arb_strategy
(
    id                  bigint auto_increment comment 'id'
        primary key,
    diff_rate_id        bigint                 not null comment 'arb_diff_rate 表ID',
    user_id             bigint                 not null comment '用户ID',
    platform            varchar(64) default '' not null comment '平台 binance、huobi、okx',
    option_choose       varchar(64)            not null comment '方向 positive, reverse',
    coin                varchar(64)            not null comment '币种',
    from_market         varchar(64) default '' not null comment 'From 市场',
    from_symbol         varchar(64) default '' not null comment 'From 交易对',
    from_price_truncate tinyint     default 2  not null comment '价格小数点保留位数',
    from_amt_truncate   tinyint     default 2  not null comment '数量小数点保留位数',
    to_market           varchar(64) default '' not null comment 'To 市场',
    to_symbol           varchar(64) default '' not null comment 'To 交易对',
    to_price_truncate   tinyint     default 2  not null comment '价格小数点保留位数',
    to_amt_truncate     tinyint     default 2  not null comment '数量小数点保留位数',
    from_to_desc        varchar(256)           not null comment 'from->to',
    to_from_desc        varchar(256)           not null comment 'to->from',
    option_open         decimal(20, 4)         not null comment '入场阀值',
    option_close        decimal(20, 4)         not null comment '出场阀值',
    option_amt          decimal(20, 4)         not null comment '操作数量 ',
    contract_mul        int                    not null comment '合约面值、合约乘数',
    margin_mul          int         default 1  not null comment '杠杆倍数',
    fok_diff            decimal(20, 4)         null comment 'FOK单子冗余处理，最新成交价格+-FOK',
    spot_fee            decimal(20, 6)         not null comment '现货手续费',
    futures_fee         decimal(20, 6)         not null comment 'U本位合约手续费',
    delivery_fee        decimal(20, 6)         not null comment '币本位合约手续费',
    doing_status        tinyint     default 0  not null comment '策略状态 0、不执行 1、执行 2、已完成',
    created             int         default 0  null comment '创建时间',
    updated             int         default 0  null comment '更新时间',
    bak                 varchar(255)           null comment '备注'
)
    comment '期现套利策略表' charset = utf8mb4;

create table arbitrage.arb_strategy_ex
(
    id                  bigint auto_increment comment 'id'
        primary key,
    user_id             bigint                 not null comment '用户ID',
    platform            varchar(64) default '' not null comment '平台 binance、okx、huobi',
    option_choose       varchar(64)            not null comment '方向 positive, reverse',
    arb_strategy_id     bigint                 not null comment 'arb_strategy 表ID',
    coin                varchar(64) default '' not null comment '币种 BTC、ETH',
    market              varchar(64) default '' not null comment '市场 spot、futures、delivery',
    symbol              varchar(64) default '' not null comment '交易对 BTCUSDT、BTCUSDT_230630',
    option_type         varchar(64)            not null comment 'positive
    操作类型, spot_buy, transfer_spot_to_delivery, delivery_sell, delivery_buy, transfer_delivery_to_spot, spot_sell
    reverse
    操作类型, spot_sell, transfer_spot_to_futures, futures_buy, futures_sell, transfer_futures_to_spot, spot_buy',
    option_status       tinyint     default 0  null comment '操作状态 0. 未完成、1. 已完成',
    option_amount       decimal(20, 4)         null comment '操作数量 现货、U本位期货代表数量，币本位代表合约张数',
    option_executed_amt decimal(20, 8)         null comment '已经执行数量',
    current_order_id    varchar(64)            null comment '当前执行操作的订单ID',
    created             int         default 0  null comment '创建时间',
    updated             int         default 0  null comment '更新时间',
    bak                 varchar(255)           null comment '备注'
)
    comment '预生成策略执行表' charset = utf8;

create table arbitrage.arb_strategy_ex_info
(
    id                 bigint auto_increment comment 'id'
        primary key,
    user_id            bigint                 not null comment '用户ID',
    platform           varchar(64) default '' not null comment '平台 binance、okx、huobi',
    option_choose      varchar(64)            not null comment '方向 positive, reverse',
    arb_strategy_id    bigint                 not null comment '策略列表ID',
    arb_strategy_ex_id bigint                 not null comment '策略执行表ID',
    coin               varchar(64)            not null comment '币种',
    market             varchar(64)            null comment '交易市场 spot、futures、delivery, transfer',
    symbol             varchar(64)            not null comment '交易对/币种...',
    option_type        varchar(64)            not null comment 'positive
    操作类型, spot_buy, transfer_spot_to_delivery, delivery_sell, delivery_buy, transfer_delivery_to_spot, spot_sell
    reverse
    操作类型, spot_sell, transfer_spot_to_futures, futures_buy, futures_sell, transfer_futures_to_spot, spot_buy',
    price              decimal(20, 4)         not null comment '价格',
    amount             decimal(20, 8)         not null comment '数量 现货、U本位期货代表数量，币本位代表合约张数',
    executed_amt       decimal(20, 8)         null comment '真实执行数量',
    order_id           varchar(64)            null comment '委托单ID',
    is_ok              tinyint     default 0  not null comment '0 未完成 1 已完成 2 已失效',
    created            int         default 0  null comment '创建时间',
    updated            int         default 0  null comment '更新时间',
    bak                varchar(255)           null comment '备注'
)
    comment '策略执行记录表' charset = utf8;

