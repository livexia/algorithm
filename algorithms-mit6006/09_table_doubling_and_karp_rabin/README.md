## 9. Table Doubling, Karp-Rabin

时间：20210415

记录：

1. table size
    - if n > m, grow table
    - 
2. Grow table
    - make table of size m'
    - build new hash f'
    - rehash: for item in T: T'.insert(item)
3. table doubling
    - m' = 2m;
    - cost of inserts = Θ(1 + 2 + 4 + 8 +...+ n) = Θ(n)
4. Amortization
    - T(n) amorized
    - cheap on average
    - table doubling insert/delete "T(n) amorized" = Θ(1)
5. Deletion
    1. if m = 2 * n then m = m / 2. slow. 8 <--> 9, will shrink and double
    2. if m = 4 * n then shrink m = m / 2

6. python list (resizable array): use table doubling

7. String matching
    - given two strings s & t, does s occur as a substring of t?
    1. simple algorithm:
        - check each substring with the length of s
        - any( s == t[i:i + len(s)] for i in range(len(t) - len(s)))
        - O(len(t) * len(s))
    2. Rolling hash ADT
        - r.append(c), add char c to then end of x
        - r.skip(c) delete first char of x (assuming it is c)
        - r maintains a string x
        - r(): hash value of x = h(x)
    3. Karp-Rabin algorithm
        ```python
        for c in s:
            rs.append(c)
        for c in t[:len(s)]:
            rt.append(c)
        if rs() == rt():
            if s == t[i-len(s) + 1: i + 1]:
                # found match
            else:
                # O(1) when probability <= 1/len(s)

        for i in range(len(s), len(t)):
            rt.skip(t[i - len(s)])
            rt.append(t[i])
            if rs() == rt():
                if s == t[i-len(s) + 1: i + 1]:
                    # found match
                    # O(s)
                else:
                    # O(1) when probability <= 1/len(s)
        ```
        1. rolling hash: rs/rt
            - division method: h(k) = k mode m
            - m: random prime >= len(s)
            - prehash: treat x as multidigit number u in base a of alphabet size
            - append(c): (u · a + ord(c)) mod p = [(u mod m) · a + ord(c)] mod m
            - skip(c): [u - ord(c) · (a**(|u| - 1) mod m)] mod m = [(u mode m) - ord(c) · ((a**(|u| - 1) mod m))] mod m
