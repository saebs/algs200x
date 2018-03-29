# Uses python3
import sys

def get_fibonacci_huge_naive(n, m):
    if n <= 1:
        return n

    previous = 0
    current  = 1

    for _ in range(n - 1):
        previous, current = current, previous + current

    return current % m

def get_fibonacci_huge_fast(n, m):
    if n <= 1 :
        return n
    
    previous = 0
    current = 1
    pissano_l = 2 
    for i in range(n-1):
        #print(fibs)
        previous, current = current%m, (previous + current)%m
        pissano_l += 1 
        if  previous == 0 and current == 1:
            print("pissano")
            print("count:",i)
            break
    print(pissano_l)
    n = n% pissano_l

    previous = 0
    current  = 1

    for _ in range(n - 1):
        previous, current = current, previous + current

    return current % m

if __name__ == '__main__':
    input = sys.stdin.read()
    n, m = map(int, input.split())
    print(get_fibonacci_huge_fast(n, m))
