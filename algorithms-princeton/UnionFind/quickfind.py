# encoding = utf-8

# Quick Find
class QF:
    id = []
    count = 0
    def __init__(self, n):
        self.id = list(range(n))
        self.count = n

    def union(self, p, q):
        if self.connected(p,q):
            pass
        else:
            old_p = self.id[p]
            for i in range(self.count):
                if self.id[i] == old_p:
                    self.id[i] = self.id[q]

    def connected(self, p, q):
        if self.id[p] == self.id[q]:
            return 1
        return 0
