# unictrl

A unifi controller terminal client written in Rust. Because I need one and I want to learn Rust.

## Usage
Reads address, username and password for the controller from environment variables:
```
UNICTRL_HOST=http://controller
UNICTRL_USERNAME=User
UNICTRL_PASSWORD=super-secret-password
```

Once those are set simply run the 
```
cargo build
cargo run
```


## Ubiquiti API reference
https://ubntwiki.com/products/software/unifi-controller/api
