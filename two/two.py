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

def solve2():
    total = 0
    for line in sys.stdin:
        nums = list(map(int, line.strip().split()))
        for i in range(len(nums)):
            for j in range(len(nums)):
                if i == j:
                    continue
                if nums[i] % nums[j] == 0:
                    total += nums[i] // nums[j]
    return total

if __name__ == '__main__':
    print(solve2())
