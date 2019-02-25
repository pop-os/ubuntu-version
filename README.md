# ubuntu-version

This Rust crate exists to convert Ubuntu versions to codenames, and vice versa.

```rust
use ubuntu_version::{Codename, Version};

fn main() {
    let version = "18.04 LTS".parse::<Version>().unwrap();
    let codename = "bionic".parse::<Codename>().unwrap();

    let version = Version::from(codename);
    let codename = Codename::from(version);

    println!("{} ({})", version, codename);
    println!("{}", <&'static str>::from(codename));
}
```
