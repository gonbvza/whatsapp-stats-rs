# WhatsApp Chat Statistics Dashboard

This project analyzes exported **WhatsApp chat logs** and generates an **interactive HTML dashboard** with detailed statistics about conversations.  
It is written in **Rust** and uses [Askama](https://github.com/djc/askama) for templating.

---

## Features

- Count **messages per user** and **words per user**
- Track **phrase frequency** across participants
- Identify the **top speaker per hour** (0–23)
- Extract **word frequencies** (case-insensitive)
- Calculate:
  - Most active user
  - Most active hour
  - Average words per message
  - Average messages per user
  - Longest message length
- Generate a **dashboard** (`output/index.html`) with:
  - User activity chart
  - Word frequency chart
  - Hourly speaker schedule
  - Summary statistics

---

## Installation

### 1. Clone the repository
```bash
git clone https://github.com/yourusername/whatsapp-stats.git
cd whatsapp-stats
```

2. Build the project

`cargo build --release`

3. Run tests

`cargo test`

## Usage

    Export your WhatsApp chat from the app:

        Go to Chat → Export chat (without media recommended).

        Save the exported .txt file.

Run the analysis:

`cargo run -- --file path/to/chat.txt html`

Open the generated dashboard:

`open output/index.html`

### Commands
Run `--help` to see more commands
