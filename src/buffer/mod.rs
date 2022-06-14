use crate::buffer::base_ring_buffer::BaseRingBuffer;

pub mod base_ring_buffer;

// pub const capacitys: [usize; 6] = [100, 1000, 10_000, 100_000, 1_000_000, 10_000_000];

enum RingBufferTy<T> {
    Buffer100(BaseRingBuffer<100, T>),
    Buffer1000(BaseRingBuffer<1000, T>),
    Buffer10_000(BaseRingBuffer<10_000, T>),
    Buffer100_000(BaseRingBuffer<100_000, T>),
    Buffer1_000_000(BaseRingBuffer<1_000_000, T>),
    Buffer10_000_000(BaseRingBuffer<10_000_000, T>),
}

pub struct RingBuffer<T> {
    buffer: RingBufferTy<T>,
}
impl<T> RingBuffer<T> {
    // pub fn default() -> Self {
    //     Self {
    //         buffer: RingBufferTy::Buffer_100(BaseRingBuffer::<100, T>::default()),
    //     }
    // }
}
