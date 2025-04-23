# x86_64-pc-windows-gnu
# x86_64-pc-windows-msvc

# x86_64-apple-darwin
# aarch64-apple-darwin

# x86_64-unknown-linux-gnu

# all: macos windows linux
#
# macos:
# 	cargo build --target=x86_64-apple-darwin --release
# 	cargo build --target=aarch64-apple-darwin --release
#
# windows:
# 	cargo build --target=x86_64-pc-windows-gnu --release
#
# linux:
# 	cargo build --target=x86_64-unknown-linux-gnu --release
#
#

PREFIX=target
BIN_NAME=youtubby
APP_NAME=Youtubby
DMG_DIR=${PREFIX}/${APP_NAME}.dmg
APP_DIR=${PREFIX}/${APP_NAME}.app
CONTENTS_DIR=${APP_DIR}/Contents
RESOURCES_DIR=${CONTENTS_DIR}/Resources
MACOS_DIR=${CONTENTS_DIR}/MacOS
APP_BIN=${MACOS_DIR}/${BIN_NAME}

all: osx-app

osx-bin:
	cargo rustc --release

osx-app: osx-bin

	rm -rf ${APP_DIR}
	mkdir -p ${CONTENTS_DIR}
	mkdir -p ${MACOS_DIR}
	mkdir -p ${RESOURCES_DIR}
	cp -v target/release/${BIN_NAME} ${APP_BIN}
	cp -v assets/youtubby.icns ${RESOURCES_DIR}/AppIcon.icns
	cp -v assets/Info.plist ${CONTENTS_DIR}

osx-run: osx-app
	open ${APP_DIR}
