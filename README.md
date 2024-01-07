# River Ring Buffer

A simple ring buffer implemented in Rust.

## Example usage

``` Rust
    let mut buffer = RingBuffer::new(3);

    buffer.put(1)?;
    buffer.put(2)?;
    buffer.put(3)?;

    buffer.read(); // Returns Some(1)
```
