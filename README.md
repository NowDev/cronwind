# Cronwind

A simple and flexible cron job runner written in Rust that supports both command execution and HTTP requests.

## Features

- ✨ Simple JSON configuration
- 🕒 Cron-style scheduling
- 🔄 HTTP request jobs
- 💻 Command execution jobs
- 📝 Logging support
- 🔒 Type-safe configuration

## Installation

1. Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/)

2. Clone the repository:
```bash
git clone https://github.com/NowDev/cronwind.git
cd cronwind
```

3. Build the project:
```bash
cargo build --release
```

The binary will be available at `target/release/cronwind`

## Usage

1. Create a `config.json` file with your job definitions
2. Run the program:
```bash
./cronwind
```

To run in daemon mode (background):
```bash
./cronwind --daemon
```

> Note: Service/systemd support coming soon!

## Configuration

Jobs are defined in `config.json`. Each job requires:
- `name`: A unique identifier for the job
- `schedule`: Cron expression (seconds minutes hours day_of_month month day_of_week)
- `kind`: Type of job ("command" or "request")
- `config`: Job-specific configuration

### Example Configuration

```json
{
  "jobs": [
    {
      "name": "echo-job",
      "schedule": "* * * * * *",
      "kind": "command",
      "config": {
        "command": "echo 'Hello, world!'"
      },
      "outputs": [
        {
          "kind": "file",
          "config": {
            "path": "echo.log"
          }
        }
      ]
    },
    {
      "name": "github-api-check",
      "schedule": "* 30 * * * *",
      "kind": "request",
      "config": {
        "url": "https://api.github.com"
      },
      "outputs": []
    }
  ]
}
```

### Cron Schedule Format

The schedule uses the following format:
```
sec min hour day_of_month month day_of_week
```

Examples:
- `* * * * * *` - Every second
- `0 */5 * * * *` - Every 5 minutes
- `0 0 * * * *` - Every hour
- `0 0 0 * * *` - Every day at midnight

## License

This project is licensed under the MIT License - see the LICENSE file for details.
