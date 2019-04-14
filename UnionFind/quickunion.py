# encoding = utf-8

# Quick Union
class QU:
    id = []
    count = 0
    def __init__(self, n):
        self.id = list(range(n))
        self.count = n

    def root(self, p):
        while p != self.id[p]:
            p = self.id[p]
        return p

    def union(self, p, q):
        if self.connected(p,q):
            pass
        else:
            self.id[self.root(p)] = self.root(q)

    def connected(self, p, q):
        if self.root(p) == self.root(q):
            return 1
        return 0
