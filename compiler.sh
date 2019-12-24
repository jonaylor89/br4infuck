#!/usr/bin/env sh

cargo run $1 --compile && ./assemble.sh temp.s && ./a.out