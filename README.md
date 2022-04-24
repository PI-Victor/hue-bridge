Hue Bridge API
---

[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]  

[crates-badge]: https://img.shields.io/crates/v/hue-bridge  
[crates-url]: https://crates.io/crates/hue-bridge  
[docs-badge]: https://docs.rs/hue-bridge/badge.svg  
[docs-url]: https://docs.rs/hue-bridge  

This crate provides async bindings for Hue Bridge API.  

At the time of writing this, the Hue Bridge API is being migrated to API v2,
but not all endpoints have been implemented. So this crate utilizes the
implemented resources from v2 and plugs the rest with v1.
