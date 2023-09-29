export RUST_LOG=trace
export BEE_LOG_FORMAT=json
curl $1 -sG | cargo run -r -q --example=bee-jsparser-demo
