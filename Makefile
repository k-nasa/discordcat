# x86_64-unknown-linux-gnu
# x86_64-apple-darwin
# x86_64-pc-windows-gnu

TARGET:= x86_64-pc-windows-gnu
BIN_NAME:=discordcat.exe
CRATE_NAME:discordcat
MISC:= README.md LICENSE
DIRNAME:=${CRATE_NAME}_${TARGET}

build:
	cargo build
run:
	cargo run
install:
	cargo install --path . -f
upload:
	cargo publish

release_all:
	rm -rf dist/
	make release TARGET=x86_64-pc-windows-gnu    BIN_NAME=discordcat.exe
	make release TARGET=x86_64-apple-darwin      BIN_NAME=discordcat
	make release TARGET=x86_64-unknown-linux-gnu BIN_NAME=discordcat

.PHONY: release
release:
	cross build --target ${TARGET} --release
	mkdir -p ${DIRNAME}
	\
	cp ./target/${TARGET}/release/${BIN_NAME} ${DIRNAME}
	cp ${MISC} ${DIRNAME}
	\
	mkdir -p dist
	tar czf dist/${DIRNAME}.tar.gz ${DIRNAME}
	rm -rf ${DIRNAME}
