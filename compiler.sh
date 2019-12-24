#!/usr/bin/env sh

cargo run $1 > temp.s && ./assemble.sh temp.s && ./a.out