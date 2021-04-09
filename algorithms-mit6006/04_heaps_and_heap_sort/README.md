## 4. Heaps and Heap Sort

时间：20210408

记录：

1. Priority queue
    1. ADT是什么
    2. 哪些操作
2. Heap: Priority queue 的一种实现
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