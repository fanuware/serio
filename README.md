# Serial Communication

Cross-platform command-line utility for connecting to and interacting
with a device's serial console over a serial port.

## Build and Execute

### Run locally

```sh
cargo run -- --port=/dev/ttyUSB0 --baudrate 115200
```

### Build Release

```sh
cargo build --release
cargo build --target x86_64-pc-windows-gnu --release
```

### Use Container

```sh
docker build -t rust-build .
docker run -v=$(pwd):/src -it rust-build bash
```