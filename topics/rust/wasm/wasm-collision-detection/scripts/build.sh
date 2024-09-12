
echo "Compile to wasm"
wasm-pack build --release

echo "Stripping debug symbols"
wasm-strip pkg/wasm_collision_detection_bg.wasm

echo "Optimizing for size"
wasm-opt -o pkg/wasm_collision_detection_bg2.wasm -Oz pkg/wasm_collision_detection_bg.wasm
