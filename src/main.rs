use bytes::BytesMut;
use dashmap::DashMap;
use regex::Regex;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// --- ১. ডাইনামিক রুল এবং বাইপাস ইঞ্জিন ---
pub struct BypassEngine {
    bypass_rules: Vec<Regex>,
}

impl BypassEngine {
    pub fn new() -> Self {
        let rules = vec![
            Regex::new(r"example-bank\.com$").unwrap(),
            Regex::new(r"checkout\.stripe\.com$").unwrap(),
            Regex::new(r"accounts\.google\.com$").unwrap(),
        ];
        BypassEngine { bypass_rules: rules }
    }

    pub fn should_bypass(&self, domain: &str) -> bool {
        self.bypass_rules.iter().any(|re| re.is_match(domain))
    }
}

// --- ২. হাই-পারফরম্যান্স ট্রাফিক লগার ---
pub struct HighQualityLogger {
    log_cache: Arc<DashMap<String, Vec<u8>>>,
}

impl HighQualityLogger {
    pub fn new() -> Self {
        HighQualityLogger {
            log_cache: Arc::new(DashMap::new()),
        }
    }

    pub fn log_payload(&self, session_id: &str, data: &[u8]) {
        println!("[Logger] Session: {} | Logged {} bytes", session_id, data.len());
        self.log_cache.insert(session_id.to_string(), data.to_vec());
    }
}

// --- ৩. জিরো-কপি স্ট্রিম প্রসেসর ---
pub async fn process_stream_zero_copy(
    inbound: TcpStream,
    outbound: TcpStream,
    session_id: String,
    logger: Arc<HighQualityLogger>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let (mut ri, mut wi) = inbound.into_split();
    let (mut ro, mut wo) = outbound.into_split();

    let logger_cloned = logger.clone();
    let session_id_cloned = session_id.clone();

    let client_to_server = tokio::spawn(async move {
        let mut buffer = BytesMut::with_capacity(16384);
        loop {
            let n = match ri.read_buf(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            logger_cloned.log_payload(&session_id_cloned, &buffer[..n]);
            if wo.write_all(&buffer[..n]).await.is_err() {
                break;
            }
            buffer.clear();
        }
    });

    let server_to_client = tokio::spawn(async move {
        let mut buffer = BytesMut::with_capacity(16384);
        loop {
            let n = match ro.read_buf(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            logger.log_payload(&session_id, &buffer[..n]);
            if wi.write_all(&buffer[..n]).await.is_err() {
                break;
            }
            buffer.clear();
        }
    });

    tokio::try_join!(client_to_server, server_to_client)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let bypass_engine = Arc::new(BypassEngine::new());
    let _logger = Arc::new(HighQualityLogger::new());

    let target_domain = "example-bank.com";

    if bypass_engine.should_bypass(target_domain) {
        println!("[Engine] Danger Domain Detected! Bypassing: {}", target_domain);
    } else {
        println!("[Engine] Safe to Inspect: {}", target_domain);
    }
}
