/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        // 将新元素添加到数组末尾
        self.items.push(value);
        self.count += 1;
        
        // 上浮操作，维护堆的性质
        let mut idx = self.count;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            // 如果当前节点满足堆的性质，则停止
            // 对于最小堆：当前节点 >= 父节点
            // 对于最大堆：当前节点 >= 父节点
            if !(self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                break;
            }
            // 否则交换当前节点和父节点
            self.items.swap(idx, parent_idx);
            idx = parent_idx;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        
        // 检查左子节点是否存在
        if left_idx > self.count {
            return 0; // 没有子节点
        }
        
        // 检查右子节点是否存在
        if right_idx > self.count {
            // 只有左子节点
            return left_idx;
        }
        
        // 两个子节点都存在，返回根据比较器"更优"的那个
        // 对于最小堆 (a < b)，返回较小的那个
        // 对于最大堆 (a > b)，返回较大的那个
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx
        } else {
            right_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		if self.count == 0 {
            return None;
        }
        
        // 保存根节点的值
        let result = std::mem::take(&mut self.items[1]);
        self.count -= 1;
        
        if self.count == 0 {
            // 如果堆为空，移除占位符元素
            self.items.pop();
            return Some(result);
        }
        
        // 将最后一个元素移到根位置
        let last_item = self.items.pop().unwrap();
        self.items[1] = last_item;
        
        // 下沉操作，维护堆的性质
        let mut idx = 1;
        while self.children_present(idx) {
            let child_idx = self.smallest_child_idx(idx);
            if child_idx == 0 {
                break;
            }
            // 如果当前节点已经满足堆的性质，则停止
            if !(self.comparator)(&self.items[child_idx], &self.items[idx]) {
                break;
            }
            self.items.swap(idx, child_idx);
            idx = child_idx;
        }
        
        Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}