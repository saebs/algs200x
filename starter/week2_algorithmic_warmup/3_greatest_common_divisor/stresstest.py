import random
from gcd import *

def stress_test():
    """ testing to maximum constraint """
    import random
    while True:
        a = random.randint(1,200000)
        b = random.randint(1,200000)
        result1 = gcd_naive(a, b)
        result2 = gcd_euclid(a, b)
        if result1 == result2:
            print("OK")
        else:
            print("Wrong Answer", result1, result2)
            return

if __name__ == "__main__":
    stress_test()

