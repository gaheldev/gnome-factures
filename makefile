.PHONY: run install

run:
	cargo run

install:
	cargo build --release
	install -D target/release/gnome-factures ~/.local/bin/gnome-factures
	install -D assets/factures.desktop ~/.local/share/applications/gnome-factures.desktop
