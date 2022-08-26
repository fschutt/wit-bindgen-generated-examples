#!/bin/bash
set -x

WIT_BINDGEN_SOURCEPATH=../wit-bindgen

rm -rf wasmer3

create_project() {
    project_name=$1
    cargo new --edition 2018 --name wit_bindgen_examples_"$project_name" --lib "$project_name"
    rm -rf "$project_name"/src

    cat >> "$project_name"/Cargo.toml<< EOF

[lib]
path = "lib.rs"
EOF

    cat >> "$project_name"/lib.rs<< EOF
pub mod codegen;
pub mod runtime;
EOF

    mkdir -p "$project_name/codegen"
    mkdir -p "$project_name/runtime"

    touch "$project_name/codegen/mod.rs"
    touch "$project_name/runtime/mod.rs"
}

create_project wasmer3

cat >> wasmer3/Cargo.toml<< EOF

[dependencies.wasmer]
git = "https://github.com/wasmerio/wasmer"
branch = "master"

[dependencies.wit-bindgen-wasmer]
git = "https://github.com/wasmerio/wit-bindgen"
branch = "wasmer"
EOF

generate() {
    imports_wit=$1
    exports_wit=$2
    wasmer3_destination_dir=$3

    mkdir -p "$wasmer3_destination_dir"

        if [ -e "$imports_wit" ]; then
            cargo \
                run \
                --manifest-path "$WIT_BINDGEN_SOURCEPATH"/Cargo.toml \
                wasmer \
                --rustfmt \
                --import "$imports_wit" \
                --out-dir "$wasmer3_destination_dir" \
                || return 1
            sed -r -i 's/^\s+$//' "$wasmer3_destination_dir"/bindings.rs
            mv "$wasmer3_destination_dir/bindings.rs" "$wasmer3_destination_dir/imports.rs"
            cp "$imports_wit" "$wasmer3_destination_dir"/imports.wit
        fi

        if [ -e "$exports_wit" ]; then
              cargo \
                run \
                --manifest-path "$WIT_BINDGEN_SOURCEPATH"/Cargo.toml \
                wasmer \
                --rustfmt \
                --export "$exports_wit" \
                --out-dir "$wasmer3_destination_dir" \
                || return 1
            sed -r -i 's/^\s+$//' "$wasmer3_destination_dir"/bindings.rs
            mv "$wasmer3_destination_dir/bindings.rs" "$wasmer3_destination_dir/exports.rs"
            cp "$exports_wit" "$wasmer3_destination_dir"/exports.wit
        fi
}

for dir in "$WIT_BINDGEN_SOURCEPATH"/tests/runtime/*; do
    example_name=$(basename "$dir")
    host_file="$dir"/host-wasmer.rs
    imports_wit="$dir"/exports.wit
    exports_wit="$dir"/imports.wit
    wasmer3_destination_dir=wasmer3/runtime/"${example_name/-/_}"

    if [ -e "$host_file" ]; then
        generate \
            "$imports_wit" \
            "$exports_wit" \
            "$wasmer3_destination_dir" \
            || continue

        echo "pub mod ${example_name/-/_};" >> wasmer3/runtime/mod.rs

        cat >> "$wasmer3_destination_dir"/mod.rs<< EOF
pub mod exports;
pub mod imports;
EOF

    fi
done

for file in "$WIT_BINDGEN_SOURCEPATH"/tests/codegen/*.wit; do
    example_name=$(basename "$file" .wit)
    imports_wit="$file"
    exports_wit="$file"
    wasmer3_destination_dir=wasmer3/codegen/"${example_name/-/_}"

    generate \
        "$imports_wit" \
        "$exports_wit" \
        "$wasmer3_destination_dir" \
        || continue

    echo "pub mod ${example_name/-/_};" >> wasmer3/codegen/mod.rs

    cat >> "$wasmer3_destination_dir"/mod.rs<< EOF
pub mod exports;
pub mod imports;
EOF
done

(cd wasmer3 && cargo fmt)
