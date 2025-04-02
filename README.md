# Lark Notifier

A command-line tool for sending notifications to Lark (Feishu) with support for message signing and keyword highlighting.

## Features

- Send notifications to Lark via webhook
- Support for signed messages with HMAC-SHA256
- Highlight keywords in message content
- Docker support for easy deployment

## Installation

### Using Cargo

```bash
cargo install --git https://github.com/7a6163/lark-notifier
```

### Using Docker

```bash
# Pull from Docker Hub
docker pull 7a6163/lark-notifier:latest

# Or pull from GitHub Container Registry
docker pull ghcr.io/7a6163/lark-notifier:latest
```

## Usage

### Command Line

```bash
# Basic usage
lark-notifier --webhook-url "https://open.larksuite.com/open-apis/bot/v2/hook/your-webhook" \
              --title "Notification Title" \
              --content "This is a notification message"

# With secret for signed messages
lark-notifier --webhook-url "https://open.larksuite.com/open-apis/bot/v2/hook/your-webhook" \
              --secret "your-secret" \
              --title "Notification Title" \
              --content "This is a notification message"

# With keyword highlighting
lark-notifier --webhook-url "https://open.larksuite.com/open-apis/bot/v2/hook/your-webhook" \
              --title "Notification Title" \
              --content "This is a notification with highlighted keywords" \
              --keywords "notification,highlighted"
```

### Docker

```bash
docker run -e LARK_WEBHOOK_URL="https://open.larksuite.com/open-apis/bot/v2/hook/your-webhook" \
           -e LARK_SECRET="your-secret" \
           7a6163/lark-notifier \
           --title "Notification Title" \
           --content "This is a notification message"
```

## Environment Variables

- `LARK_WEBHOOK_URL`: The webhook URL for your Lark bot
- `LARK_SECRET`: The secret for signed messages (optional)

## Building from Source

```bash
# Clone the repository
git clone https://github.com/7a6163/lark-notifier.git
cd lark-notifier

# Build
cargo build --release

# Run
./target/release/lark-notifier --help
```

## License

MIT
