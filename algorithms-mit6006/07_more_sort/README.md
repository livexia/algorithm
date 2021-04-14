## 7. Counting Sort, Radix Sort, Lower Bounds for Sorting

时间：20210414

记录：

1.  Comparison model:
    1. input item are black boxes(ADTS)
    2. only operations allowed are comparisions(<, >, ==)
    3. time cost = number of comparison
2. Decision tree: comparison algorithm
    1. comparision
    2. outcomes
    3. resulting
3. Searching lower bound
    1. tree must have log2(n) height
    2. decision tree is binary and must have >= n leafs for each answer
    3. so height is must >= log2(n)
4. sorting lower bound
    1. decision tree is binary and must have >= n! leafs for each answer
    2. => so height >= log2(n!) => log2(n(n-1)(n-1)...1)
    3. => sum of log2(i) 1 <= i < n
    4. => >= sum of log2(i) n/2 <= i < n
    5. => >= sum of log2(n/2) n/2 <= i < n
    6. => = sum of (log2(n) - 1) n/2 <= i < n
    7. => = n/2*log2(n) - n/2
5. Linear-time Sorting (Integer sorting)
    1. fitting in a word
    2. 0, 1, 2,...,k - 1
    3. can do more than comparison
    4. if k = n**O(1), can sort in O(n) time
6. counting sort: O(n+k), need small size of k
7. radix sort: k = n**O(1) => k <= n\**100
    1. imagine each integer as base b
    2. #digits = d = logb(k) + 1
    3. min when b == n
    4. sort (all n items) by least significant digit → can extract in O(1) time
    5.  · · ·
    6. sort by most significant digit → can extract in O(1) time
        - sort must be stable: preserve relative order of items with the same key
        - =⇒ don’t mess up previous sorting
