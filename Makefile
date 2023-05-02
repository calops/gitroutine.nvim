all: debug

prep:
	[ -d "lua" ] || mkdir lua

debug: prep
	cargo build --features=neovim-nightly
	cp ./target/debug/libgitroutine.so ./lua/gitroutine.so

release: prep
	cargo build --release --features=neovim-nightly
	cp ./target/release/libgitroutine.so ./lua/gitroutine.so
