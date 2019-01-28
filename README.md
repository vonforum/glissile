# Glissile

**Glissile** will be a macro for generating GLSL source code.
Currently it just re-inserts its contents as a `r#`-delimited string.

Example:

```rust
extern crate glissile;
use glissile::glsl;

let frag_code = glsl! {
    precision mediump float;
    varying vec2 texCoord;

    void main() {
        gl_FragColor = vec4(texCoord, 0., 1.);
    }
};
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
