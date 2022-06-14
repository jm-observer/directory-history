use anyhow::{bail, Result};
use std::array::from_fn;
/// 对于top_index、bottom_index，值0是一个特殊值。无数据时，二者都为0。且只有无数据时，二者才会是0。
/// top_index、bottom_index为索引的位置皆有值。
///
pub struct BaseRingBuffer<const CAPACITY: usize, T> {
    capacity: usize,
    last_index: usize,
    first_index: usize,
    data: Box<[Option<T>]>,
}
impl<const CAPACITY: usize, T: Clone> Default for BaseRingBuffer<CAPACITY, T> {
    fn default() -> Self {
        Self {
            capacity: CAPACITY,
            last_index: 0,
            first_index: 0,
            data: vec![None::<T>; CAPACITY].into_boxed_slice(),
        }
    }
}

impl<const CAPACITY: usize, T: Clone> BaseRingBuffer<CAPACITY, T> {
    pub fn push_vec(&mut self, datas: Vec<T>) -> Result<()> {
        for data in datas.into_iter() {
            if let Err(e) = self.push(data) {
                bail!("buffer is full");
            }
        }
        Ok(())
    }
    pub fn push(&mut self, data: T) -> std::result::Result<(), T> {
        if self.last_index == 0 {
            self.last_index = 1;
            self.first_index = 1;
            self.data[1] = Some(data);
        } else {
            let mut top_tmp = self.last_index + 1;
            if top_tmp == CAPACITY {
                top_tmp = 1;
            }
            if top_tmp == self.first_index {
                return Err(data);
            } else {
                self.last_index = top_tmp;
                self.data[self.last_index] = Some(data);
            }
        }
        Ok(())
    }
    pub fn fisrt_out(&mut self) -> Option<T> {
        let index = self.first_index;
        // 相等意味着，只有1个数据
        if self.last_index == self.first_index {
            // 相等且等于0，则无数据
            if self.last_index == 0 {
                return None;
            }
            self.last_index = 0;
            self.first_index = 0;
        } else {
            self.first_index += 1;
            if self.first_index == CAPACITY {
                self.first_index = 1;
            }
        }
        self.data[index].take()
    }
    pub fn last_out(&mut self) -> Option<T> {
        let index = self.last_index;
        // 相等意味着，只有1个数据
        if self.last_index == self.first_index {
            // 相等且等于0，则无数据
            if self.last_index == 0 {
                return None;
            }
            self.last_index = 0;
            self.first_index = 0;
        } else {
            self.last_index -= 1;
            if self.last_index == 0 {
                self.last_index = CAPACITY - 1;
            }
        }
        self.data[index].take()
    }
}
