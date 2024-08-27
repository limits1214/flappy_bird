name="web"
flag="debug" # debug or release

# RUSTFLAGS="--cfg=web_sys_unstable_apis" 
cargo build $( [ "$flag" = "release" ] && echo "--release" ) --target wasm32-unknown-unknown
wasm-bindgen --out-name $name \
  --out-dir wasm \
  --target web ../target/wasm32-unknown-unknown/${flag}/$name.wasm