struct RingBuffer<T> {
    buffer: Vec<T>,
    write_index: usize,
    read_index: usize,
    capacity: usize,
}

impl<T> RingBuffer<T>
where
    T: Default,
    T: Copy,
{
    /// Create a new ring buffer with the given capacity
    pub fn new(capacity: usize) -> Self {
        RingBuffer {
            capacity: capacity + 1,
            buffer: vec![T::default(); capacity + 1],
            write_index: 0,
            read_index: 0,
        }
    }

    /// Read a value from the buffer
    ///
    /// Returns `None` if the buffer is empty.
    pub fn read(&mut self) -> Option<T> {
        if self.read_index == self.write_index {
            return None;
        }

        let value = self.buffer[self.read_index];
        self.read_index = (self.read_index + 1) % self.buffer.len();

        Some(value)
    }

    /// Write a value to the buffer
    ///
    /// If the buffer is full an error is returned.
    pub fn put(&mut self, arg: T) -> Result<(), &'static str> {
        if (self.write_index + 1) % self.capacity == self.read_index {
            return Err("Buffer is full");
        }

        self.buffer[self.write_index] = arg;
        self.write_index = (self.write_index + 1) % self.capacity;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_none_when_ring_buffer_empty() {
        let mut buffer: RingBuffer<i32> = RingBuffer::new(10); // Allocate a new ring buffer with capacity 10

        assert_eq!(None, buffer.read()); // Read from the buffer
    }

    #[test]
    fn it_returns_values_in_fifo_order() -> Result<(), &'static str> {
        let mut buffer = RingBuffer::new(10);

        buffer.put(1)?;
        buffer.put(2)?;
        buffer.put(3)?;
        buffer.put(4)?;
        buffer.put(5)?;

        assert_eq!(Some(1), buffer.read());
        assert_eq!(Some(2), buffer.read());
        assert_eq!(Some(3), buffer.read());
        assert_eq!(Some(4), buffer.read());
        assert_eq!(Some(5), buffer.read());

        Ok(())
    }

    #[test]
    fn it_returns_error_when_buffer_is_full() {
        let mut buffer = RingBuffer::new(3);

        buffer.put(1).unwrap();
        buffer.put(2).unwrap();
        buffer.put(3).unwrap();

        assert_eq!(Err("Buffer is full"), buffer.put(4));
    }

    #[test]
    fn it_overwrites_oldest_value_when_buffer_is_full() -> Result<(), &'static str> {
        let mut buffer = RingBuffer::new(3);

        buffer.put(1)?;
        buffer.put(2)?;
        buffer.put(3)?;

        assert_eq!(Some(1), buffer.read());

        buffer.put(4)?;

        assert_eq!(Some(2), buffer.read());
        assert_eq!(Some(3), buffer.read());
        assert_eq!(Some(4), buffer.read());

        Ok(())
    }
}
