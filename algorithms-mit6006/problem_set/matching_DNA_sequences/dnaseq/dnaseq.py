#!/usr/bin/env python2.7

import unittest
from dnaseqlib import *
from collections import defaultdict

### Utility classes ###

# Maps integer keys to a set of arbitrary values.
class Multidict:
    # Initializes a new multi-value dictionary, and adds any key-value
    # 2-tuples in the iterable sequence pairs to the data structure.
    def __init__(self, pairs=[]):
        self.multdict = defaultdict(list)
        for pair in pairs:
            self.put(pair[0], pair[1])

    # Associates the value v with the key k.
    def put(self, k, v):
        return self.multdict[k].append(v)

    # Gets any values that have been associated with the key k; or, if
    # none have been, returns an empty sequence.
    def get(self, k):
        return self.multdict[k]

# Given a sequence of nucleotides, return all k-length subsequences
# and their hashes.  (What else do you need to know about each
# subsequence?)
def subsequenceHashes(seq, k):
    try:
        start = 0
        s = ""
        for _ in range(k):
            s += next(seq)
        rh = RollingHash(s)
        while True:
            yield rh.curhash, (start, s)
            prev_item = s[0]
            next_item = next(seq)
            s = s[1:] + next_item
            rh.slide(prev_item, next_item)
            start += 1
    except StopIteration:
        return

# Similar to subsequenceHashes(), but returns one k-length subsequence
# every m nucleotides.  (This will be useful when you try to use two
# whole data files.)
def intervalSubsequenceHashes(seq, k, m):
    try:
        start = 0
        while True:
            s = ""
            for _ in range(k):
                s += next(seq)
            rh = RollingHash(s)
            yield rh.current_hash(), (start, s)
            for _ in range(m - k):
                next(seq)
            start += m
    except StopIteration:
        return

# Searches for commonalities between sequences a and b by comparing
# subsequences of length k.  The sequences a and b should be iterators
# that return nucleotides.  The table is built by computing one hash
# every m nucleotides (for m >= k).
def getExactSubmatches(a, b, k, m):
    seq_table = Multidict(intervalSubsequenceHashes(a, k, m))
    for hash_value, (start_b, sub_b) in subsequenceHashes(b, k):
        for start_a, sub_a in seq_table.get(hash_value):
            if sub_a != sub_b:
                continue
            yield start_a, start_b


if __name__ == '__main__':
    if len(sys.argv) != 4:
        print('Usage: {0} [file_a.fa] [file_b.fa] [output.png]'.format(sys.argv[0]))
        sys.exit(1)

    # The arguments are, in order: 1) Your getExactSubmatches
    # function, 2) the filename to which the image should be written,
    # 3) a tuple giving the width and height of the image, 4) the
    # filename of sequence A, 5) the filename of sequence B, 6) k, the
    # subsequence size, and 7) m, the sampling interval for sequence
    # A.
    compareSequences(getExactSubmatches, sys.argv[3], (500,500), sys.argv[1], sys.argv[2], 8, 100)
