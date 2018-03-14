# Uses python3
""" 
Problem Description:
....................
Task.           Given two integers a and b, find their greatest common divisor
Input Format.   the two integers a , b are given in same line seperated by space
Constraints.    1 <= a, b <= 2*10exp9
Output Format.  Output GCD(a, b)

"""

import sys


def gcd_naive(a, b):
    current_gcd = 1
    for d in range(2, min(a, b) + 1):
        if a % d == 0 and b % d == 0:
            if d > current_gcd:
                current_gcd = d
    return current_gcd

def gcd_euclid(a, b):
    if b == 0:
        return a
    a = a%b
    return gcd_euclid(b, a)
    

if __name__ == "__main__":
    input = sys.stdin.read()
    a, b = map(int, input.split())
    print(gcd_euclid(a, b))
