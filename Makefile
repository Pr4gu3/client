linux:
	cargo rustc --release
	ls -lh target/release/client
windows:
# In order to cross-compile from arch linux to windows, you may run these commands \
$ rustup target add x86_64-pc-windows-gnu \
$ pacman -S mingw-w64
	cargo rustc --release --target x86_64-pc-windows-gnu
	ls -lh target/x86_64-pc-windows-gnu/release/client.exe