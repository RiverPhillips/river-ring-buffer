# River Ring Buffer

[![Crates.io Version](https://img.shields.io/crates/v/river-ring-buffer)](https://crates.io/crates/river-ring-buffer)
[![docs.rs (with version)](https://img.shields.io/docsrs/river-ring-buffer/0.4.0)](https://docs.rs/river-ring-buffer/0.4.0/river_ring_buffer/)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/riverphillips/river-ring-buffer/rust.yml)
![Crates.io License](https://img.shields.io/crates/l/river-ring-buffer)

A simple ring buffer implemented in Rust.

A ring buffer is a high performant data structure that is great for buffering streams of data.

My motivation for building this was to understand ring buffers
as they're used extensively in new Linux Kernel feature such as eBPF and IO Uring.

## Example usage

``` Rust
use river_ring_buffer::RingBuffer;

fn main() -> Result<(), &'static str> {
    let mut buffer = RingBuffer::new(3);

    buffer.put(1)?;
    buffer.put(2)?;
    buffer.put(3)?;

    buffer.read(); // Returns Some(1)
}
```

An error is returned when the buffer is full.

```Rust
use river_ring_buffer::RingBuffer;

fn main() -> Result<(), &'static str> {
    let mut buffer = RingBuffer::new(3);

    buffer.put(1)?;
    buffer.put(2)?;
    buffer.put(3)?;
    buffer.put(4)?; // Returns Err("Buffer is full")   
}
```
