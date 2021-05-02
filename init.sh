# プロジェクト作成時のみ
# cargo install cargo-generate

curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cd mandelbrot
wasm-pack build