#!/usr/bin/env python3

import sys

def solve():
    total = 0
    for line in sys.stdin:
        spl = list(map(int, line.strip().split()))
        largest = max(spl)
        smallest = min(spl)
        total += largest - smallest
    return total

if __name__ == '__main__':
    print(solve())
