#!/bin/bash

WIT_BINDGEN_SOURCEPATH=../wit-bindgen

mkdir -p wasmer2
mkdir -p wasmer3

for dir in "$WIT_BINDGEN_SOURCEPATH"/tests/runtime/*; do
    example_name=$(basename "$dir")
    host_file="$dir"/host-wasmer.rs
    imports_file="$dir"/exports.wit
    exports_file="$dir"/imports.wit

    mkdir -p "wasmer2/$example_name/"
    mkdir -p "wasmer3/$example_name/"

    if [ -e "$host_file" ]; then
        if [ -e "$imports_file" ]; then
            wit-bindgen \
                wasmer \
                --import "$imports_file" \
                --out-dir "wasmer2/$example_name"
            mv "wasmer2/$example_name/bindings.rs" "wasmer2/$example_name/imports.rs"

            cargo \
                run \
                --manifest-path "$WIT_BINDGEN_SOURCEPATH"/Cargo.toml \
                wasmer \
                --import "$imports_file" \
                --out-dir "wasmer3/$example_name"
            mv "wasmer3/$example_name/bindings.rs" "wasmer3/$example_name/imports.rs"
        fi

        if [ -e "$exports_file" ]; then
            wit-bindgen \
                wasmer \
                --export "$exports_file" \
                --out-dir "wasmer2/$example_name"
            mv "wasmer2/$example_name/bindings.rs" "wasmer2/$example_name/exports.rs"

            cargo \
                run \
                --manifest-path "$WIT_BINDGEN_SOURCEPATH"/Cargo.toml \
                wasmer \
                --export "$exports_file" \
                --out-dir "wasmer3/$example_name"
            mv "wasmer3/$example_name/bindings.rs" "wasmer3/$example_name/exports.rs"
        fi
    fi
done
