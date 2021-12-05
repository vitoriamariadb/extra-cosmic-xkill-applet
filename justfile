build-release:
    cargo build --release

install:
    cp target/release/cosmic-xkill-applet /usr/local/bin/
