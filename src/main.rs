use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{Engine as _, engine::general_purpose};

type HmacSha256 = Hmac<Sha256>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Lark webhook URL (if not provided, use LARK_WEBHOOK_URL env var)
    #[arg(short, long)]
    webhook_url: Option<String>,

    /// Lark app secret for signed messages (if not provided, use LARK_SECRET env var)
    #[arg(short, long)]
    secret: Option<String>,

    /// Message title
    #[arg(short, long)]
    title: String,

    /// Message content
    #[arg(short, long)]
    content: String,

    /// Keywords to highlight (comma separated)
    #[arg(short, long)]
    keywords: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LarkMessage {
    msg_type: String,
    content: LarkContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    sign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LarkContent {
    post: LarkPost,
}

#[derive(Serialize, Deserialize, Debug)]
struct LarkPost {
    zh_cn: LarkPostContent,
}

#[derive(Serialize, Deserialize, Debug)]
struct LarkPostContent {
    title: String,
    content: Vec<Vec<LarkTextContent>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LarkTextContent {
    tag: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
}

fn generate_sign(timestamp: u64, secret: &str) -> String {
    // timestamp + key 做 sha256, 再进行 base64 编码
    let string_to_sign = format!("{}\n{}", timestamp, secret);
    
    let mac = HmacSha256::new_from_slice(string_to_sign.as_bytes())
        .expect("HMAC can take key of any size");
    
    let result = mac.finalize().into_bytes();
    general_purpose::STANDARD.encode(result)
}

fn get_env_or_arg(arg: Option<String>, env_name: &str) -> Result<String, String> {
    match arg {
        Some(value) => Ok(value),
        None => match env::var(env_name) {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Missing {} environment variable or command argument", env_name)),
        }
    }
}

fn process_content_with_keywords(content: &str, keywords: &[String]) -> Vec<LarkTextContent> {
    if keywords.is_empty() {
        return vec![LarkTextContent {
            tag: "text".to_string(),
            text: content.to_string(),
            href: None,
        }];
    }

    let mut result = Vec::new();
    let mut remaining = content.to_string();

    for keyword in keywords {
        if remaining.contains(keyword) {
            let parts: Vec<&str> = remaining.splitn(2, keyword).collect();
            
            if !parts[0].is_empty() {
                result.push(LarkTextContent {
                    tag: "text".to_string(),
                    text: parts[0].to_string(),
                    href: None,
                });
            }
            
            // Add the keyword as a highlighted text
            result.push(LarkTextContent {
                tag: "a".to_string(),
                text: keyword.to_string(),
                href: Some("".to_string()),  // Empty href for highlighting only
            });
            
            remaining = parts[1].to_string();
        }
    }
    
    if !remaining.is_empty() {
        result.push(LarkTextContent {
            tag: "text".to_string(),
            text: remaining,
            href: None,
        });
    }
    
    result
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    let webhook_url = get_env_or_arg(args.webhook_url, "LARK_WEBHOOK_URL")?;
    let secret = get_env_or_arg(args.secret, "LARK_SECRET").ok();
    
    let client = Client::new();
    
    let keywords: Vec<String> = match args.keywords {
        Some(k) => k.split(',').map(|s| s.trim().to_string()).collect(),
        None => Vec::new(),
    };
    
    let content_elements = process_content_with_keywords(&args.content, &keywords);
    
    let mut message = LarkMessage {
        msg_type: "post".to_string(),
        content: LarkContent {
            post: LarkPost {
                zh_cn: LarkPostContent {
                    title: args.title,
                    content: vec![content_elements],
                },
            },
        },
        sign: None,
        timestamp: None,
    };
    
    if let Some(secret_key) = secret {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        
        let sign = generate_sign(timestamp, &secret_key);
        
        message.sign = Some(sign);
        message.timestamp = Some(timestamp.to_string());
    }

    let res = client.post(&webhook_url)
        .json(&message)
        .send()
        .await?;
    
    if res.status().is_success() {
        println!("Successfully sent notification to Lark");
    } else {
        eprintln!("Failed to send notification: {}", res.status());
        eprintln!("Response: {}", res.text().await?);
    }

    Ok(())
}

