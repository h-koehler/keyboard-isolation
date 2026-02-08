# make wasm_run
cargo build --release --features bevy/webgpu --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./webbuild/out/ --target web ./target/wasm32-unknown-unknown/release/keyboard_isolation.wasm
cp -r assets ./webbuild/
cp index.html ./webbuild/index.html
# Copy the contents of index.html (see below), then run the following to replace keyboard_isolation with your crate name from Cargo.toml
PKG_NAME=$(grep '^name = ' Cargo.toml | awk -F '"' '{print $2}' | head -n 1)
sed -i '' "s/\/out\/keyboard_isolation.js/\/out\/${PKG_NAME}.js/g" webbuild/index.html

zip -r webbuild.zip webbuild
