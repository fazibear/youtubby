#!/bin/bash

set -e

OS=macos
MACHINE=x86_64
MACHINE=arm64

PREFIX=target
BIN_NAME=youtubby
APP_NAME=Youtubby
DMG_DIR=$PREFIX/$APP_NAME.dmg
APP_DIR=$PREFIX/$APP_NAME.app
CONTENTS_DIR=$APP_DIR/Contents
RESOURCES_DIR=$CONTENTS_DIR/Resources
MACOS_DIR=$CONTENTS_DIR/MacOS
APP_BIN=$MACOS_DIR/$BIN_NAME

echo "Cleaning ..."
rm -rf $APP_DIR
cargo clean --release

echo "Creating app directory structure"
mkdir -p $CONTENTS_DIR
mkdir -p $MACOS_DIR
mkdir -p $RESOURCES_DIR

echo "Compiling..."
cargo rustc --frozen --release

echo "Copying files"
cp -v target/release/$BIN_NAME $APP_BIN
cp -v assets/youtubby.icns $RESOURCES_DIR/AppIcon.icns
cp -v assets/Info.plist $CONTENTS_DIR

open $APP_DIR

# echo "Creating dmg"
# mkdir $DMG_DIR
# mv $APP_DIR $DMG_DIR
# ln -s /Applications $DMG_DIR/Applications
# rm -rf $DMG_DIR/.Trashes
#
# FULL_NAME=$BIN_NAME-$OS-$MACHINE
#
# hdiutil create uploads/$FULL_NAME.dmg -srcfolder $DMG_DIR -ov
# rm -rf $DMG_DIR
