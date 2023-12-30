export RUST_LOG=trace
export BEE_LOG_FORMAT=json
cat | cargo run -rq --example=jsparser
