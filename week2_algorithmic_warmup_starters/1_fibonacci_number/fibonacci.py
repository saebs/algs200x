# Uses python3
import math
import random

def calc_fib(n):
    if (n <= 1):
        return n

    return calc_fib(n - 1) + calc_fib(n - 2)


def calc_fib_fast(n):
    """ uses one line formula derived by induction"""
    part1 = 1/2 + (math.sqrt(5)/2) 
    print(part1)
    part2 = 1/2 - (math.sqrt(5)/2)
    print(part2)
    fn = 1/math.sqrt(5)*(part1**n - part2**n)
    return int(fn)

def stress_test():
    """ testing to maximum constraint """
    while True:
        n = random.randint(0,45)
        result1 = calc_fib(n)
        result2 = calc_fib_fast(n)
        if result1 == result2:
            print("OK")
        else:
            print("Wrong Answer", result1, result2)
try:
    n = int(input())
except:
     pass
"""
if n < 0 or n > 45:
    #stress_test()
    exit()
"""
print(calc_fib_fast(n)%10)


