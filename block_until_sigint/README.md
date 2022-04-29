
## Description

A simple easy to use wrapper around Ctrl-C signal.

## Example usage

```rust

use anyhow::{Context, Result, Ok};
use block_until_sigint::block;

#[tokio::main]
async fn main() -> Result<()> {

    block(async {
        println!("Hello, world!");
    }).await?;
    
    Ok(())
}

```

## Contributing

1. Fork it
2. Download your fork to your PC
3. Create your feature branch (`git checkout -b my-new-feature`)
4. Make changes and add them (`git add .`)
5. Commit your changes (`git commit -m 'Add some feature'`)
6. Push to the branch (`git push origin my-new-feature`)
7. Create new pull request

## Security Vulnerabilities

If you discover a security vulnerability, please send an e-mail to [pefish@qq.com](mailto:pefish@qq.com). All security vulnerabilities will be promptly addressed.

## License

This project is licensed under the [Apache License](LICENSE).
