.PHONY: run install

run:
	cargo run

install:
	cargo build --release
	install -D target/release/facture-gui ~/.local/bin/gnome-factures
	install -D assets/factures.desktop ~/.local/share/applications/gnome-factures.desktop
