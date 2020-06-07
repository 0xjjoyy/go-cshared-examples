go build -o library/libawesome.so -buildmode=c-shared ../awesome.go

bindgen library/libawesome.h -o src/bindings.rs 

Add above each 'extern "C" {' to src/bindings.rs
#[link(name = "awesome", kind="dylib")]

cargo build

DYLD_LIBRARY_PATH=./library target/debug/client_rust
