# encoding = utf-8

# Weighted Quick Union
class WQUPC:
    id = []
    sz = []
    count = 0
    def __init__(self, n):
        self.id = list(range(n))
        self.sz = [1] * n
        self.count = n

    def root(self, p):
        while p != self.id[p]:
            self.id[p] = self.id[self.id[p]]
            p = self.id[p]
        return p

    def union(self, p, q):
        if self.connected(p,q):
            pass
        else:
            rootp = self.root(p)
            rootq = self.root(q)
            if self.sz[rootp] >= self.sz[rootq]:
                self.id[rootq] = rootp
                self.sz[rootp] += self.sz[rootq]
            else:
                self.id[rootp] = rootq
                self.sz[rootq] += self.sz[rootp]

    def connected(self, p, q):
        if self.root(p) == self.root(q):
            return 1
        return 0
