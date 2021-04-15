## 8. Hashing with Chaining

时间：20210415

记录：

1. Dictionary/HashMap: ADT
    - insert
    - delete
    - search: O(1) with high possibility
2. Motivation:
    - database
    - compilers & interpreters
    - network router
    - substring search
    - rsync
    - cryptography
3. simple approach
    1. Direct-access table
        - store items in array
        - indexed by key
        - cons:
            1. keys may not be nonneg integers
            2. gigantic memory hog
3. prehash: fix keys may not be nonneg integers
    - map keys to noneg integers
    - math: string of bits
    - python prehash:
    - rust prehash:
4. hashing: reducing memory space
    - reduce of all keys(integers) down to reasonable size m for table
    - collision: h(ki) == h(kj) but ki != kj
5. chaining: fix hashing collision
    - linked list of collision elements in each of hash table
    - worst case Θ(n) time
6. simple uniform hashing(false assumption)[SUHA](https://en.wikipedia.org/wiki/SUHA_(computer_science))
    - running time O(1)
7. hash functions:
    1. division method: h(k) = k mod m
    2. multiplication method: h(k) = (k * a) mod 2**w] >> (w - r)
        - w bit machine
        - a random integers
        - r: m = 2**r
    3. universal hashing: h(k) = [(a*k + b) mod p] mod m
        - a, b random in {0, 1,..., p - 1}
        - p is prime number big than size or universe
        - worst-case keys k1 != k2, Pr{h(k1) = h(k2)} = 1 / m
    