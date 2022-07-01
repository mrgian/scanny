# Scanny

Simple domain and port scanner wirtten in Rust

## Usage
```
scanny <domain.com>
```

Or if you want compile from source, install Rust (don't need to tell you how to do this), and then:
```
git clone https://github.com/mrgian/scanny.git
cd scanny
cargo run -- <domain.com>
```

## To do
Scanny is only able to scan for subdomains and open ports at the moment, but I will add many more features in the future, such as:

- Make it multithreaded
- Vulnerability scanner
- And many more
