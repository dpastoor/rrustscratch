// We need to forward routine registration from C to Rust
// to avoid the linker removing the static library.

void R_init_dynamicdataframes_extendr(void *dll);

void R_init_dynamicdataframes(void *dll) {
    R_init_dynamicdataframes_extendr(dll);
}
