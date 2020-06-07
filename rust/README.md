go build -o library/libawesome.so -buildmode=c-shared ../awesome.go

cargo build

DYLD_LIBRARY_PATH=./library target/debug/client_rust
