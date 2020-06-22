#!/usr/bin/env bash
set -euxo pipefail # https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/

export YEW_DEBUGGER_PORT=80;
export YEW_DEBUGGER_HOST=echo.websocket.org;

(cd yew \
  && cargo test --target wasm32-unknown-unknown --features wasm_test,dev \
  && cargo test --doc --features doc_test,wasm_test,yaml,msgpack,cbor,toml \
  && cargo test --doc --features doc_test,wasm_test,yaml,msgpack,cbor,toml \
    --features std_web,agent,services --no-default-features)

(cd yew-functional && cargo test --target wasm32-unknown-unknown)

(cd yew-macro \
  && cargo test --test macro_test \
  && cargo test --test derive_props_test \
  && cargo test --doc)

(cd yew-router && cargo test)
(cd yew-router-macro && cargo test)
(cd yew-router-route-parser && cargo test)

(cd yew-stdweb && cargo test --target wasm32-unknown-unknown --features wasm_test)

(cd yewtil && cargo test)

(cd yew-components && cargo test)

(cd devtools && cargo test)
(cd devtools-extension && cargo test --features logic_test)
