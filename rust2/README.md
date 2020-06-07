go build -o library/libawesome.so -buildmode=c-shared ../awesome.go

bindgen library/libawesome.h -o src/bindings.rs 

cargo build

DYLD_LIBRARY_PATH=./library target/debug/client_rust
