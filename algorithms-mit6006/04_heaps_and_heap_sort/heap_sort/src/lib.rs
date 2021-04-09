use std::fmt::Debug;

/**
Heap: Priority queue 的一种实现
1. An **array** visualized as a **nearly complete binary tree*
2. root = a[0]
3. parent(i) = a[i / 2]
4. left(i) = a[2 * i], right(i) = a[2 * i + 1]
5. max-heap: 父节点的值大于子节点的值
6. 如何从一个未排序的数组中构建max/min-heap
7. 两个方法：
    - build_max_heap
    - max_heapify
8. 复杂度计算
*/

#[derive(Clone)]
pub struct MaxHeap<T> {
    data: Vec<T>,
}

impl<T> MaxHeap<T>
where
    T: Clone + Copy + PartialEq + Eq + Ord + Debug,
{
    fn new() -> Self {
        MaxHeap { data: Vec::new() }
    }

    fn from_vec(data: &Vec<T>) -> Self {
        let mut heap = MaxHeap { data: data.clone() };
        heap.build_max_heap();
        heap
    }

    fn build_max_heap(&mut self) {
        let n = self.len();
        for i in (0..n / 2).rev() {
            self.max_heapify(i)
        }
    }

    fn max_heapify(&mut self, i: usize) {
        let n = self.len();
        let l = 2 * i + 1;
        let r = 2 * i + 2;
        let mut largest = i;
        if l < n && self.data[l] > self.data[i] {
            largest = l;
        }
        if r < n && self.data[r] > self.data[largest] {
            largest = r;
        }
        if largest != i {
            self.swap(i, largest);
            self.max_heapify(largest);
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.data.swap(a, b)
    }

    // fn pop(&mut self) -> Option<T>{
    //     self.data.pop()
    // }

    // fn sort(&self) -> Vec<T> {
    //     let mut heap = self.clone();
    //     let mut result = vec![];
    //     while !heap.is_empty() {
    //         let n = heap.len();
    //         heap.swap(0, n - 1);
    //         result.push(heap.pop().unwrap());
    //         heap.max_heapify(0);
    //     }
    //     result
    // }

    fn pop_max(&mut self) -> Option<T>{
        let n = self.len();
        self.swap(0, n - 1);
        let result = self.data.pop();
        self.max_heapify(0);
        result
    }

    fn sort(&self) -> Vec<T> {
        let mut heap = self.clone();
        let mut result = vec![];
        while !heap.is_empty() {
            result.push(heap.pop_max().unwrap());
        }
        result
    }

    fn push(&mut self, a: T) {
        self.data.push(a);
        let mut cur = self.len() - 1;
        while cur != 0 {
            let parent = (cur - 1) / 2;
            if self.data[cur] > self.data[parent] {
                self.swap(cur, parent);
                cur = parent;
            } else {
                break;
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::MaxHeap;
    use rand::{Rng, thread_rng};

    #[test]
    fn it_works() {
        let a = vec![-5, 10, 20, 1, -10, 14, 12, 9, 78, 0];
        let mut heap = MaxHeap::from_vec(&a);
        let result = heap.sort();
        
        let mut a = a;
        a.sort();
        a.reverse();

        assert_eq!(result, a);

        heap.push(100);
        assert_eq!(heap.pop_max().unwrap(), 100);
    }

    #[test]
    fn rand_test() {
        let mut nums = [0i32; 100000];
        thread_rng().fill(&mut nums[..]);
        let nums = nums.to_vec();

        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        sorted_nums.reverse();

        let result = MaxHeap::from_vec(&nums).sort();
        assert_eq!(result, sorted_nums);
    }

}
