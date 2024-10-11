/*
	heap
	This question requires you to implement a binary heap function
*/
// a little bit hard
// 为什么vector下标也是从1开始呢，害我错了好久

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Clone
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 这里在创建的时候就加入了一个T类型的default值，第一次push数组时，是往下标为1处插入数据才正确
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
        self.items.push(value);
        self.count += 1;
        if self.len() == 1 { // 只有1个结点
            return;
        }
        let mut child_idx = self.count; // 获取末尾结点下标
        let mut parent_idx = self.parent_idx(child_idx);
        // 上升
        while !(self.comparator)(&self.items[parent_idx],&self.items[child_idx]) { // 当父子结点不满足特性就交换
            // 父节点下移
            let tmp = self.items[parent_idx].clone();
            self.items[parent_idx] = self.items[child_idx].clone();
            self.items[child_idx] = tmp.clone();
            if parent_idx == 1{
                break; // 当父结点是堆顶时，意味着已经交换完毕，可以退出
            }
            child_idx = parent_idx;
            parent_idx = self.parent_idx(child_idx); // 获取父结点下标
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
        let l_idx = self.left_child_idx(idx);
        let r_idx = self.right_child_idx(idx);
        if r_idx > self.len() { // 右孩子不存在
            l_idx // 默认返回左孩子，此时可能左孩子不存在
        }else { // 右孩子存在
            if (self.comparator)(&self.items[l_idx],&self.items[r_idx]){
                l_idx
            }else {
                r_idx
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone,
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

impl<T: Clone> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		if self.is_empty() {
            None
        }else {
            let result = Some(self.items[1].clone()); // 获取堆顶元素
            self.items[1] = self.items[self.count].clone(); // 最后一个元素顶替到堆顶
            self.items.pop();
            self.count -= 1;
            // 将堆顶元素向下调整
            let mut parent_idx = 1;
            let mut child_idx = self.smallest_child_idx(parent_idx); // 获取较小的儿子的下标
            while self.children_present(parent_idx) && !(self.comparator)(&self.items[parent_idx],&self.items[child_idx]) { // 当父子结点不满足特性就交换
                // 父节点下移
                let tmp = self.items[parent_idx].clone();
                self.items[parent_idx] = self.items[child_idx].clone();
                self.items[child_idx] = tmp.clone();
                parent_idx = child_idx;
                child_idx = self.smallest_child_idx(parent_idx); // 返回的孩子结点可能是不存在的结点，用于退出
            }
            result
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
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
        for i in &heap.items{
            println!("{}",i);
        }
        // assert_eq!(heap.len(), 4);
        // assert_eq!(heap.next(), Some(11));
        // assert_eq!(heap.next(), Some(9));
        // assert_eq!(heap.next(), Some(4));
        // heap.add(1);
        // assert_eq!(heap.next(), Some(2));
    }
}