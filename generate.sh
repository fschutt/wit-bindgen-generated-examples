#!/bin/bash

WIT_BINDGEN_SOURCEPATH=../wit-bindgen

mkdir -p wasmer2
mkdir -p wasmer3

generate() {
    imports_wit=$1
    exports_wit=$2
    wasmer2_destination_dir=$3
    wasmer3_destination_dir=$4

    mkdir -p "$wasmer2_destination_dir"
    mkdir -p "$wasmer3_destination_dir"

        if [ -e "$imports_wit" ]; then
            wit-bindgen \
                wasmer \
                --import "$imports_wit" \
                --out-dir "$wasmer2_destination_dir"
            mv "$wasmer2_destination_dir/bindings.rs" "$wasmer2_destination_dir/imports.rs"

            cargo \
                run \
                --manifest-path "$WIT_BINDGEN_SOURCEPATH"/Cargo.toml \
                wasmer \
                --import "$imports_wit" \
                --out-dir "$wasmer3_destination_dir"
            mv "$wasmer3_destination_dir/bindings.rs" "$wasmer3_destination_dir/imports.rs"
        fi

        if [ -e "$exports_wit" ]; then
            wit-bindgen \
                wasmer \
                --export "$exports_wit" \
                --out-dir "$wasmer2_destination_dir"
            mv "$wasmer2_destination_dir/bindings.rs" "$wasmer2_destination_dir/exports.rs"

            cargo \
                run \
                --manifest-path "$WIT_BINDGEN_SOURCEPATH"/Cargo.toml \
                wasmer \
                --export "$exports_wit" \
                --out-dir "$wasmer3_destination_dir"
            mv "$wasmer3_destination_dir/bindings.rs" "$wasmer3_destination_dir/exports.rs"
        fi
}

for dir in "$WIT_BINDGEN_SOURCEPATH"/tests/runtime/*; do
    example_name=$(basename "$dir")
    host_file="$dir"/host-wasmer.rs
    imports_wit="$dir"/exports.wit
    exports_wit="$dir"/imports.wit
    wasmer2_destination_dir=wasmer2/runtime/"$example_name"
    wasmer3_destination_dir=wasmer3/runtime/"$example_name"

    if [ -e "$host_file" ]; then
        generate \
            "$imports_wit" \
            "$exports_wit" \
            "$wasmer2_destination_dir" \
            "$wasmer3_destination_dir"
    fi
done

for file in "$WIT_BINDGEN_SOURCEPATH"/tests/codegen/*.wit; do
    example_name=$(basename "$file" .wit)
    imports_wit="$file"
    exports_wit="$file"
    wasmer2_destination_dir=wasmer2/codegen/"$example_name"
    wasmer3_destination_dir=wasmer3/codegen/"$example_name"

    generate "$imports_wit" "$exports_wit" "$wasmer2_destination_dir" "$wasmer3_destination_dir"
done
