# x86_64-pc-windows-gnu
# x86_64-pc-windows-msvc

# x86_64-apple-darwin
# aarch64-apple-darwin

# x86_64-unknown-linux-gnu

all: macos windows linux

macos:
	cargo build --target=x86_64-apple-darwin --release
	cargo build --target=aarch64-apple-darwin --release

windows:
	cargo build --target=x86_64-pc-windows-gnu --release

linux:
	cargo build --target=x86_64-unknown-linux-gnu --release
