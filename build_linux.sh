#!/bin/bash
################################################################################################
# Script Name: build_linux.sh                                                                                             #
# Description: cargo build --release for inux x86_64-linux-musl-gcc                                                                                             #
# Version:0.1
# Author: Peter.F.C.Gui
# Usage: ./build_linux.sh                                                                                             #
################################################################################################
export run_mode=dev
export OPENSSL_DIR=$(brew --prefix openssl@1.1)
export OPENSSL_LIB_DIR=$OPENSSL_DIR/lib
export OPENSSL_INCLUDE_DIR=$OPENSSL_DIR/include
export PKG_CONFIG_PATH=$OPENSSL_DIR/lib/pkgconfig:$PKG_CONFIG_PATH
export CC=x86_64-linux-musl-gcc
export CXX=x86_64-linux-musl-g++
export AR=x86_64-linux-musl-ar
# export RUSTFLAGS="-C linker=$CC"
cross build --release --target=x86_64-unknown-linux-musl
