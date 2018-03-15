import random
from lcm import *

def stress_test():
    """ testing to maximum constraint """
    import random
    while True:
        a = random.randint(1,200000)
        b = random.randint(1,200000)
        result1 = lcm_naive(a, b)
        result2 = lcm_fast(a, b)
        if result1 == result2:
            print("OK")
        else:
            print("Wrong Answer", result1, result2)
            return

if __name__ == "__main__":
    stress_test()

