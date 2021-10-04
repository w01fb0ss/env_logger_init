## 自用env_log库
### 使用

1. `Cargo.toml`添加如下：
   ```toml
   [dependencies]
   env_logger_init = { git = "https://github.com/w01fb0ss/env_logger_init" }
   log = "*"
   ```
2. 

```rust
use env_logger_init;

#[macro_use]
extern crate log;

fn main() {
    env_logger_init::init("info");
    info!("aaa");
}

```