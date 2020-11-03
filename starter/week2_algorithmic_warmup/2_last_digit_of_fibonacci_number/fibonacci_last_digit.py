# Uses python3
import sys


def get_fibonacci_last_digit_naive(n):
    if n <= 1:
        return n

    previous = 0
    current  = 1

    for _ in range(n - 1):
        previous, current = current, previous + current
    return current % 10

#sample 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, ... 

def get_fibonacci_last_digit_fast(n):
    if n <= 1:
        return n

    previous = 0
    current  = 1

    for _ in range(n - 1):
        previous, current = current,(previous + current)%10
    return current

def stress_test():
    """ testing to maximum constraint """
    import random
    while True:
        n = random.randint(0,100000000)
        print("n:",n)
        result1 = get_fibonacci_last_digit_naive(n)
        result2 = get_fibonacci_last_digit_fast(n)
        if result1 == result2:
            print("OK")
        else:
            print("Wrong Answer", result1, result2)

if __name__ == '__main__':
    input = sys.stdin.read()
    n = int(input)

    if 0 <= n <= 100000000:
       print(get_fibonacci_last_digit_fast(n))
#stress_test()


