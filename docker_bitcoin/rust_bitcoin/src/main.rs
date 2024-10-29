use chrono::{DateTime, Utc, NaiveDateTime};
use mysql::*;
use mysql::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time;
use warp::{Filter, http::Method};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::error::Error;
use bitcoincore_rpc::bitcoin::consensus::encode::serialize;



#[derive(Debug, Deserialize)]
struct BlockData {
    // 基本区块信息
    block_height: u64,
    block_hash: String,
    block_version: u32,     // 新增字段: 区块版本
    previous_block_hash: String,
    timestamp: u32,
    confirmations: u64,

    // 区块大小和难度
    block_size: u64,
    block_weight: u64,      // 新增字段: 区块权重
    difficulty: f64,

    // 交易相关信息
    transaction_count: u64,
    avg_tx_size: f64,

    // 经济相关信息
    total_supply: f64,

    // 时间和哈希率
    block_time: f64,
    block_interval: f64,    // 新增字段: 区块间隔时间
    hash_rate: f64,

    // 其他
    nonce: u32,
    merkle_root: String,
}
#[derive(Debug, Deserialize)]
struct MarketData {
    market_cap: u64,
    fully_diluted_valuation: Option<u64>,
    total_volume: u64,
    low_24h: f64,
    high_24h: f64,
    price_change_percentage_24h: f64,
    circulating_supply: f64,
    max_supply: Option<f64>,
}
#[derive(Debug, Deserialize)]
struct PriceData {
    prices: Option<Vec<[f64; 2]>>,      // [timestamp, price]
    market_caps: Option<Vec<[f64; 2]>>, // [timestamp, market_cap]
    total_volumes: Option<Vec<[f64; 2]>>, // [timestamp, volume]
}

#[derive(Debug, Serialize)]
struct PriceDataDaySend {
    timestamp: String,
    price: f64,
}

#[derive(Debug, Serialize)]
struct PriceDataWeekSend {
    timestamp: String,
    price: f64,
}

#[derive(Debug, Serialize)]
struct PriceDataMonthSend {
    timestamp: String,
    price: f64,
}

#[derive(Debug, Serialize)]
struct PriceDataYearSend {
    timestamp: String,
    price: f64,
}

#[derive(Debug, Serialize)]
struct MarketDataSend {
    market_cap: u64,
    diluted_market_cap: u64,
    volume_24h: u64,
    vol_market_cap_ratio: f64,
    low_24h: f64,
    high_24h: f64,
    price_change_24h: f64,
    circulating_supply: f64,
    max_supply: f64,
    genesis_block_date: String,
    last_updated: String,
}

#[derive(Debug, Serialize,Deserialize)]
struct BlockDataSend {
    // 基本区块信息
    block_height: u64,
    block_hash: String,
    block_version: u32,     // 新增字段: 区块版本
    previous_block_hash: String,
    timestamp: String,
    confirmations: u64,

    // 区块大小和难度
    block_size: u64,
    block_weight: u64,      // 新增字段: 区块权重
    difficulty: f64,

    // 交易相关信息
    transaction_count: u64,
    avg_tx_size: f64,

    // 经济相关信息
    total_supply: f64,

    // 时间和哈希率
    block_time: f64,
    block_interval: f64,    // 新增字段: 区块间隔时间
    hash_rate: f64,

    // 其他
    nonce: u32,
    merkle_root: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "http://host.docker.internal:8332"; // Bitcoin Core 节点的 URL
    let rpc_auth = Auth::UserPass("user1".to_string(), "123456".to_string());  // 更新为你的 Bitcoin Core 认证信息

    // 创建 Bitcoin Core 客户端
    let client = Client::new(rpc_url, rpc_auth)?;

    // MySQL 数据库连接设置
    let url = "mysql://root:MySQL000000.@mysql:3306/block_chain";
    let pool = Pool::new(url)?;  // 修复后的错误类型

    // 启用 Warp 服务器并增加 CORS 支持
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::GET, Method::POST]);

    // 创建 Warp 路由用于获取 price_day, price_week, price_month, price_year 数据
    let routes = warp::path("price_day")
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handle_get_price_data_day)
        .or(
            warp::path("price_week")
                .and(warp::get())
                .and(with_db(pool.clone()))
                .and_then(handle_get_price_data_week)
        )
        .or(
            warp::path("price_month")
                .and(warp::get())
                .and(with_db(pool.clone()))
                .and_then(handle_get_price_data_month)
        )
        .or(
            warp::path("price_year")
                .and(warp::get())
                .and(with_db(pool.clone()))
                .and_then(handle_get_price_data_year)
        )
        .or(
            warp::path("market_data")
                .and(warp::get())
                .and(with_db(pool.clone()))
                .and_then(handle_get_market_data)
        )
        .or(
            warp::path("block_data")
                .and(warp::get())
                .and(with_db(pool.clone()))
                .and_then(handle_get_block_data)
        )
        .with(cors);  // 启用 CORS

    // 启动 Warp 服务器
    tokio::spawn(async move {
        warp::serve(routes)
            .run(([0, 0, 0, 0], 3030))
            .await;
    });

    // 无限循环，每5分钟刷新一次数据
    loop {
        fetch_and_store_data("1", "price_day", &pool).await?;
        fetch_and_store_data("7", "price_week", &pool).await?;
        fetch_and_store_data("30", "price_month", &pool).await?;
        fetch_and_store_data("365", "price_year", &pool).await?;
        fetch_and_store_market_data(&pool).await?;
        fetch_and_store_block_data(&client, &pool).await?;
        time::sleep(Duration::from_secs(120)).await;
    }
}




// 用于 Warp 的数据库过滤器
fn with_db(pool: Pool) -> impl Filter<Extract = (Pool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

// 处理获取 price_day 数据的 API 请求
async fn handle_get_price_data_day(pool: Pool) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.get_conn().unwrap();

    // 保留原查询，选择表中相同间隔的 20 条记录
    let query = r#"
        WITH ranked_data AS (
            SELECT timestamp, price, ROW_NUMBER() OVER (ORDER BY timestamp) AS row_num
            FROM price_day
        ),
        total_count AS (
            SELECT COUNT(*) AS total_rows FROM price_day
        )
        SELECT timestamp, price
        FROM ranked_data, total_count
        WHERE row_num % FLOOR(total_rows / 20) = 0
        ORDER BY timestamp
        LIMIT 20;
    "#;

    let result: Vec<(String, f64)> = conn.query_map(
        query,
        |(timestamp, price): (String, f64)| {
            // 在闭包中合并时间格式化逻辑
            match NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S") {
                Ok(dt) => {
                    let formatted_timestamp = dt.format("%H:%M").to_string();
                    let formatted_price = format!("{:.2}", price).parse::<f64>().unwrap();
                    (formatted_timestamp, formatted_price)
                }
                Err(e) => {
                    eprintln!("Failed to parse timestamp {}: {:?}", timestamp, e);
                    (String::from("Invalid timestamp"), price)
                }
            }
        },
    ).unwrap();


    // 构建 PriceData_day 结构
    let data: Vec<PriceDataDaySend> = result.into_iter()
        .map(|(timestamp, price)| PriceDataDaySend { timestamp, price })
        .collect();

    Ok(warp::reply::json(&data))
}
// 处理获取 price_week 数据的 API 请求

async fn handle_get_price_data_week(pool: Pool) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.get_conn().unwrap();

    // 查询语句，选择表中相同间隔的 20 条记录
    let query = r#"
        WITH ranked_data AS (
            SELECT timestamp, price, ROW_NUMBER() OVER (ORDER BY timestamp) AS row_num
            FROM price_week
        ),
        total_count AS (
            SELECT COUNT(*) AS total_rows FROM price_week
        )
        SELECT timestamp, price
        FROM ranked_data, total_count
        WHERE row_num % FLOOR(total_rows / 20) = 0
        ORDER BY timestamp
        LIMIT 20;
    "#;

    let result: Vec<(String, f64)> = conn.query_map(
        query,
        |(timestamp, price): (String, f64)| {
            // 在闭包中合并时间格式化逻辑
            match NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S") {
                Ok(dt) => {
                    let formatted_timestamp = dt.format("%m-%d %H").to_string();
                    let formatted_price = format!("{:.2}", price).parse::<f64>().unwrap();
                    (formatted_timestamp, formatted_price)
                }
                Err(e) => {
                    eprintln!("Failed to parse timestamp {}: {:?}", timestamp, e);
                    (String::from("Invalid timestamp"), price)
                }
            }
        },
    ).unwrap();

    // 构建 PriceDataWeekSend 结构
    let data: Vec<PriceDataWeekSend> = result.into_iter()
        .map(|(timestamp, price)| PriceDataWeekSend { timestamp, price })
        .collect();

    Ok(warp::reply::json(&data))
}
// 处理获取 price_month 数据的 API 请求

async fn handle_get_price_data_month(pool: Pool) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.get_conn().unwrap();

    // 查询语句，选择表中相同间隔的 20 条记录
    let query = r#"
        WITH ranked_data AS (
            SELECT timestamp, price, ROW_NUMBER() OVER (ORDER BY timestamp) AS row_num
            FROM price_month
        ),
        total_count AS (
            SELECT COUNT(*) AS total_rows FROM price_month
        )
        SELECT timestamp, price
        FROM ranked_data, total_count
        WHERE row_num % FLOOR(total_rows / 20) = 0
        ORDER BY timestamp
        LIMIT 20;
    "#;

    let result: Vec<(String, f64)> = conn.query_map(
        query,
        |(timestamp, price): (String, f64)| {
            // 在闭包中合并时间格式化逻辑
            match NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S") {
                Ok(dt) => {
                    let formatted_timestamp = dt.format("%m-%d").to_string(); // 修改格式化，取月-日
                    let formatted_price = format!("{:.2}", price).parse::<f64>().unwrap();
                    (formatted_timestamp, formatted_price)
                }
                Err(e) => {
                    eprintln!("Failed to parse timestamp {}: {:?}", timestamp, e);
                    (String::from("Invalid timestamp"), price)
                }
            }
        },
    ).unwrap();


    // 构建 PriceDataMonthSend 结构
    let data: Vec<PriceDataMonthSend> = result.into_iter()
        .map(|(timestamp, price)| PriceDataMonthSend { timestamp, price })
        .collect();

    Ok(warp::reply::json(&data))
}

// 处理获取 price_year 数据的 API 请求
async fn handle_get_price_data_year(pool: Pool) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.get_conn().unwrap();

    // 查询语句，选择表中相同间隔的 20 条记录
    let query = r#"
        WITH ranked_data AS (
            SELECT timestamp, price, ROW_NUMBER() OVER (ORDER BY timestamp) AS row_num
            FROM price_year
        ),
        total_count AS (
            SELECT COUNT(*) AS total_rows FROM price_year
        )
        SELECT timestamp, price
        FROM ranked_data, total_count
        WHERE row_num % FLOOR(total_rows / 20) = 0
        ORDER BY timestamp
        LIMIT 20;
    "#;

    let result: Vec<(String, f64)> = conn.query_map(
        query,
        |(timestamp, price): (String, f64)| {
            // 在闭包中合并时间格式化逻辑
            match NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S") {
                Ok(dt) => {
                    let formatted_timestamp = dt.format("%Y-%m-%d").to_string(); // 格式化为 年-月-日
                    let formatted_price = format!("{:.2}", price).parse::<f64>().unwrap();
                    (formatted_timestamp, formatted_price)
                }
                Err(e) => {
                    eprintln!("Failed to parse timestamp {}: {:?}", timestamp, e);
                    (String::from("Invalid timestamp"), price)
                }
            }
        },
    ).unwrap();

    // 构建 PriceDataYearSend 结构
    let data: Vec<PriceDataYearSend> = result.into_iter()
        .map(|(timestamp, price)| PriceDataYearSend { timestamp, price })
        .collect();

    Ok(warp::reply::json(&data))
}


async fn handle_get_market_data(pool: Pool) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.get_conn().unwrap();

    // 查询语句，获取 market_data 数据
    let query = r#"
        SELECT
            market_cap,
            diluted_market_cap,
            volume_24h,
            vol_market_cap_ratio,
            low_24h,
            high_24h,
            price_change_24h,
            circulating_supply,
            max_supply,
            genesis_block_date,
            last_updated
        FROM offchain_metric
        LIMIT 1;
    "#;

    let result: Option<(u64, u64, u64, f64, f64, f64, f64, f64, f64, String, String)> = conn.query_first(query).unwrap();

    // 如果没有找到数据，返回 JSON 格式的错误消息
    if result.is_none() {
        let error_message = warp::reply::json(&serde_json::json!({
            "error": "No market data available"
        }));
        return Ok(warp::reply::with_status(error_message, warp::http::StatusCode::NOT_FOUND));
    }

    // 正常返回数据
    if let Some((market_cap, diluted_market_cap, volume_24h, vol_market_cap_ratio, low_24h, high_24h, price_change_24h, circulating_supply, max_supply, genesis_block_date, last_updated)) = result {
        let market_data_send = MarketDataSend {
            market_cap,
            diluted_market_cap,
            volume_24h,
            vol_market_cap_ratio,
            low_24h,
            high_24h,
            price_change_24h,
            circulating_supply,
            max_supply,
            genesis_block_date,
            last_updated,
        };

        // 返回带有状态码的 JSON 响应
        let json_response = warp::reply::json(&market_data_send);
        return Ok(warp::reply::with_status(json_response, warp::http::StatusCode::OK));
    }

    // 默认返回错误
    let error_message = warp::reply::json(&serde_json::json!({
        "error": "No market data available"
    }));
    Ok(warp::reply::with_status(error_message, warp::http::StatusCode::NOT_FOUND))
}

async fn handle_get_block_data(pool: Pool) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = pool.get_conn().unwrap();

    // 查询语句，获取 block_data 数据
    let query = r#"
        SELECT
            block_height,
            block_hash,
            block_version,
            previous_block_hash,
            timestamp,
            confirmations,
            block_size,
            block_weight,
            difficulty,
            transaction_count,
            avg_tx_size,
            total_supply,
            block_time,
            block_interval,
            hash_rate,
            nonce,
            merkle_root
        FROM onchain_metrics
        ORDER BY block_height DESC
    "#;

    // 使用 `query_map` 映射每一行数据
    let result: Vec<BlockDataSend> = conn.query_map(query, |row: Row| {
        serde_json::from_value(serde_json::json!({
        "block_height": row.get::<u64, _>(0),
        "block_hash": row.get::<String, _>(1),
        "block_version": row.get::<u32, _>(2),
        "previous_block_hash": row.get::<String, _>(3),
        "timestamp": row.get::<String, _>(4),
        "confirmations": row.get::<u64, _>(5),
        "block_size": row.get::<u64, _>(6),
        "block_weight": row.get::<u64, _>(7),
        "difficulty": row.get::<f64, _>(8),
        "transaction_count": row.get::<u64, _>(9),
        "avg_tx_size": row.get::<f64, _>(10),
        "total_supply": row.get::<f64, _>(11),
        "block_time": row.get::<f64, _>(12),
        "block_interval": row.get::<f64, _>(13),
        "hash_rate": row.get::<f64, _>(14),
        "nonce": row.get::<u32, _>(15),
        "merkle_root": row.get::<String, _>(16),
    })).unwrap()
    }).unwrap();

    // 如果没有找到数据，返回 JSON 格式的错误消息
    if result.is_empty() {
        let error_message = warp::reply::json(&serde_json::json!({
            "error": "No block data available"
        }));
        return Ok(warp::reply::with_status(error_message, warp::http::StatusCode::NOT_FOUND));
    }

    // 返回带有状态码的 JSON 响应
    let json_response = warp::reply::json(&result);
    Ok(warp::reply::with_status(json_response, warp::http::StatusCode::OK))
}






// 从 API 获取并存储数据
async fn fetch_and_store_data(days: &str, table: &str, pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.coingecko.com/api/v3/coins/bitcoin/market_chart?vs_currency=usd&days={}",
        days
    );

    // 从 API 获取数据
    let response = reqwest::get(&url).await?;
    let price_data: PriceData = response.json().await?;

    // 检查数据是否可用，否则记录错误并跳过
    if price_data.prices.is_none() || price_data.market_caps.is_none() || price_data.total_volumes.is_none() {
        eprintln!("Error: Missing required field in the response for table '{}'. Skipping this fetch.", table);
        return Ok(());
    }

    let mut conn = pool.get_conn()?;

    // 清空表并插入新数据
    let truncate_query = format!("TRUNCATE TABLE {}", table);
    let mut tx = conn.start_transaction(TxOpts::default())?;
    tx.exec_drop(truncate_query, ())?;

    let query = format!(
        "INSERT INTO {} (timestamp, price, market_cap, volume) VALUES (?, ?, ?, ?)",
        table
    );

    for (price, market_cap, volume) in zip_data(
        &price_data.prices.unwrap(),
        &price_data.market_caps.unwrap(),
        &price_data.total_volumes.unwrap(),
    ) {
        let timestamp = convert_timestamp(price[0] as i64);
        let price_value = price[1];
        let market_cap_value = market_cap[1] as i64;
        let volume_value = volume[1] as i64;

        tx.exec_drop(
            &query,
            (timestamp, price_value, market_cap_value, volume_value),
        )?;
    }

    tx.commit()?;

    println!("Data refreshed and inserted into {} table", table);

    Ok(())
}

// 辅助函数，压缩价格、市场值和交易量数据
fn zip_data<'a>(
    prices: &'a [[f64; 2]],
    market_caps: &'a [[f64; 2]],
    volumes: &'a [[f64; 2]],
) -> impl Iterator<Item = (&'a [f64; 2], &'a [f64; 2], &'a [f64; 2])> {
    prices.iter().zip(market_caps.iter()).zip(volumes.iter()).map(|((p, mc), v)| (p, mc, v))
}

// 将时间戳从毫秒转换为人类可读的格式
fn convert_timestamp(timestamp_ms: i64) -> String {
    let datetime: DateTime<Utc> = DateTime::<Utc>::from_timestamp_millis(timestamp_ms).unwrap();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

// 从 API 获取并打印响应
async fn fetch_and_store_market_data(pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids=bitcoin";

    // 构建请求头
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"));

    // 发送带有用户代理的请求
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await?;

    // 检查 HTTP 状态码是否成功
    if !response.status().is_success() {
        eprintln!("Failed to fetch data: HTTP {}", response.status());
        return Ok(());
    }

    // 打印原始响应内容，帮助调试
    let response_text = response.text().await?;
    //println!("API Response: {}", response_text);

    // 尝试解析 JSON
    let market_data: Vec<MarketData> = serde_json::from_str(&response_text)?;

    // 检查数据是否为空
    if market_data.is_empty() {
        eprintln!("Error: No market data available.");
        return Ok(());
    }

    // 获取第一条数据（因为我们只请求了 Bitcoin 的数据）
    let data = &market_data[0];

    // 计算 Vol/Market Cap 比率
    let vol_market_cap_ratio = data.total_volume as f64 / data.market_cap as f64;

    // 准备数据插入 MySQL
    let mut conn = pool.get_conn()?;

    // 清空表并插入新数据
    let truncate_query = "TRUNCATE TABLE offchain_metric";
    let mut tx = conn.start_transaction(TxOpts::default())?;
    tx.exec_drop(truncate_query, ())?;

    let insert_query = r#"
        INSERT INTO offchain_metric (
            market_cap,
            diluted_market_cap,
            volume_24h,
            vol_market_cap_ratio,
            low_24h,
            high_24h,
            price_change_24h,
            circulating_supply,
            max_supply,
            genesis_block_date
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#;

    // 使用 unwrap_or 处理 Option 值并提供默认值
    tx.exec_drop(
        insert_query,
        (
            data.market_cap,
            data.fully_diluted_valuation.unwrap_or(0),  // 如果为 None，使用 0 作为默认值
            data.total_volume,
            vol_market_cap_ratio,
            data.low_24h,
            data.high_24h,
            data.price_change_percentage_24h,
            data.circulating_supply,
            data.max_supply.unwrap_or(0.0),  // 如果为 None，使用 0.0 作为默认值
            "2009-01-03", // Genesis Block Date
        ),
    )?;

    tx.commit()?;

    println!("Market data refreshed and inserted into database");

    Ok(())
}

async fn fetch_and_store_block_data(client: &Client, pool: &Pool) -> Result<(), Box<dyn Error>> {
    // 获取最新区块高度
    let best_block_height = client.get_block_count()?;

    for height in (best_block_height - 29)..=best_block_height {
        println!("Fetching block data for height: {}", height);

        // 获取区块哈希
        let block_hash = client.get_block_hash(height)?;

        // 获取区块
        let block = client.get_block(&block_hash)?;

        // 获取区块头
        let block_header = client.get_block_header(&block_hash)?;

        // 计算区块权重
        let block_weight = block.txdata.iter().map(|tx| serialize(tx).len() as u64).sum();

        // 获取区块版本
        let block_version_bytes = serialize(&block_header.version);
        let block_version = u32::from_le_bytes([block_version_bytes[0], block_version_bytes[1], block_version_bytes[2], block_version_bytes[3]]);


        // 获取区块时间戳
        let timestamp = block_header.time;

        // 获取区块确认数
        let confirmations = best_block_height - height + 1;

        // 计算区块大小
        let block_size = serialize(&block).len() as u64;

        // 获取区块难度
        let difficulty = client.get_difficulty()?;

        // 获取交易数量
        let transaction_count = block.txdata.len() as u64;

        // 计算平均交易大小
        let avg_tx_size = if transaction_count > 0 {
            block_size as f64 / transaction_count as f64
        } else {
            0.0
        };

        // 比特币总供应量
        let total_supply = 6.25 * height as f64;

        // 获取前一个区块的头部
        let previous_block_header = client.get_block_header(&block_header.prev_blockhash)?;

        // 计算区块时间间隔
        let block_time = if block_header.time >= previous_block_header.time {
            (block_header.time - previous_block_header.time) as f64 / 60.0
        } else {
            0.0 // 如果当前区块时间小于前一个区块，时间间隔设置为 0 或其他合适的值
        };

        // 计算哈希率
        let hash_rate = if block_time > 0.0 {
            (difficulty * 2f64.powf(32.0)) / block_time
        } else {
            0.0
        };

        // 创建 BlockData 实例
        let block_data = BlockData {
            block_height: height,
            block_hash: block_hash.to_string(),
            block_version,
            previous_block_hash: block_header.prev_blockhash.to_string(),
            timestamp,
            confirmations,
            block_size,
            block_weight,
            difficulty,
            transaction_count,
            avg_tx_size,
            total_supply,
            block_time,
            block_interval: block_time,
            hash_rate,
            nonce: block_header.nonce,
            merkle_root: block_header.merkle_root.to_string(),
        };

        // 将区块数据插入 MySQL 数据库
        let mut conn = pool.get_conn()?;
        conn.exec_drop(
            r"INSERT INTO onchain_metrics (block_height, block_hash, block_version, previous_block_hash, timestamp, confirmations,
                                           block_size, block_weight, difficulty,
                                           transaction_count, avg_tx_size,
                                           total_supply,
                                           block_time, block_interval, hash_rate,
                                           nonce, merkle_root)
              VALUES (:block_height, :block_hash, :block_version, :previous_block_hash, FROM_UNIXTIME(:timestamp), :confirmations,
                      :block_size, :block_weight, :difficulty,
                      :transaction_count, :avg_tx_size,
                      :total_supply,
                      :block_time, :block_interval, :hash_rate,
                      :nonce, :merkle_root)
              ON DUPLICATE KEY UPDATE
                block_hash = VALUES(block_hash),
                block_version = VALUES(block_version),
                previous_block_hash = VALUES(previous_block_hash),
                timestamp = VALUES(timestamp),
                confirmations = VALUES(confirmations),
                block_size = VALUES(block_size),
                block_weight = VALUES(block_weight),
                difficulty = VALUES(difficulty),
                transaction_count = VALUES(transaction_count),
                avg_tx_size = VALUES(avg_tx_size),
                total_supply = VALUES(total_supply),
                block_time = VALUES(block_time),
                block_interval = VALUES(block_interval),
                hash_rate = VALUES(hash_rate),
                nonce = VALUES(nonce),
                merkle_root = VALUES(merkle_root)",
            params! {
                "block_height" => block_data.block_height,
                "block_hash" => block_data.block_hash,
                "block_version" => block_data.block_version,
                "previous_block_hash" => block_data.previous_block_hash,
                "timestamp" => block_data.timestamp,
                "confirmations" => block_data.confirmations,
                "block_size" => block_data.block_size,
                "block_weight" => block_data.block_weight,
                "difficulty" => block_data.difficulty,
                "transaction_count" => block_data.transaction_count,
                "avg_tx_size" => block_data.avg_tx_size,
                "total_supply" => block_data.total_supply,
                "block_time" => block_data.block_time,
                "block_interval" => block_data.block_interval,
                "hash_rate" => block_data.hash_rate,
                "nonce" => block_data.nonce,
                "merkle_root" => block_data.merkle_root,
            },
        )?;

        // 删除最旧的区块记录，保持最多 30 条记录
        conn.exec_drop(
            r"DELETE FROM onchain_metrics
              WHERE block_height < (
                SELECT block_height FROM (
                  SELECT block_height
                  FROM onchain_metrics
                  ORDER BY block_height DESC
                  LIMIT 1 OFFSET 29
                ) AS temp
              )",
            (),
        )?;
    }

    println!("Block data successfully fetched and stored.");
    Ok(())
}