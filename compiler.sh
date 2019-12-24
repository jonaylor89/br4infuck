#!/usr/bin/env sh

cargo run $1 && ./assemble.sh temp.s && ./a.out