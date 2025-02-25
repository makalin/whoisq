# WhoisQ

A simple, fast WHOIS search web application built with Rust and the Rocket framework.

## Overview

WhoisQ allows users to input a domain name and retrieve its WHOIS information quickly and efficiently. It uses Rust for high performance and Tokio for asynchronous network queries, making it a lightweight yet powerful tool for domain lookups.

## Features

- **Simple Interface**: Enter a domain name and get WHOIS results with a single click.
- **Fast Performance**: Built with Rust, leveraging its speed and efficiency.
- **Supported TLDs**: Includes .com, .net, .org, .edu, and .uk (easily extensible).
- **Error Handling**: Gracefully handles invalid domains or connection issues.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Cargo (comes with Rust)

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/makalin/whoisq.git
   cd whoisq
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the application**:
   ```bash
   cargo run
   ```

   The app will be available at `http://127.0.0.1:8000`.

## Usage

1. Open your browser and navigate to `http://127.0.0.1:8000`.
2. Enter a domain name (e.g., `google.com`) in the input field.
3. Click "Search" to view the WHOIS information.
4. Use the "Back to search" link to perform another query.

## Project Structure

```
whoisq/
├── Cargo.toml
├── README.md
├── src/
│   └── main.rs
└── static/
    └── templates/
        ├── index.hbs
        └── result.hbs
```

- **`src/main.rs`**: Main application logic, including WHOIS query handling and Rocket routes.
- **`static/templates/`**: Handlebars templates for the web interface.

## Dependencies

- [Rocket](https://rocket.rs/) - Web framework for Rust.
- [rocket_dyn_templates](https://crates.io/crates/rocket_dyn_templates) - Handlebars templating support.
- [Tokio](https://tokio.rs/) - Asynchronous runtime for network operations.
- [Serde](https://serde.rs/) - Serialization/deserialization for form handling.

## Extending TLD Support

To add support for more TLDs, modify the `get_whois_server` function in `src/main.rs` by adding new entries to the `whois_servers` HashMap. For example:

```rust
whois_servers.insert("io", "whois.nic.io");
```

## Contributing

Feel free to submit issues or pull requests to improve WhoisQ. Suggestions for additional features or performance enhancements are welcome!

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details (create one if you plan to distribute it).

## Acknowledgments

Built with ❤️ using Rust and Rocket. Inspired by the need for a fast, simple WHOIS lookup tool.
