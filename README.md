# step-id

[![crates.io](https://img.shields.io/crates/v/step-id.svg)](https://crates.io/crates/step-id)

Stable 16-char hex IDs for agent steps. Same `(run_id, index, kind)`
always produces the same id.

```rust
use step_id::id;
let i = id("run-1", 0, "tool_call");
```

Zero deps. MIT or Apache-2.0.
