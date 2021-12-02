#!/usr/bin/sh
# This script is meant to be run on Unix/Linux based systems
set -e

echo "*** Initializing WASM build environment"

if [ -z $CI_PROJECT_NAME ] ; then
   rustup update nightly
   rustup update stable
fi

rustup toolchain install nightly-2021-06-09
rustup override set nightly-2021-06-09
rustup target add wasm32-unknown-unknown --toolchain nightly-2021-06-09