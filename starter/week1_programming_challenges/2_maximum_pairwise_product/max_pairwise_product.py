# Uses python3
"""
n = int(input())
a = [int(x) for x in input().split()]
assert(len(a) == n)

"""

def max_pairwise_product(n, a):
    """ Naive but correct algorithm"""
    for i in range(0, n):
        for j in range(i + 1, n):
            if a[i] * a[j] > result:
                result = a[i] * a[j]
    return result


def max_pairwise_product_fast(n, an):
    """Fast implementation"""
    index1 = 0 
    for i in range(1,n):
        if an[i] > an[index1]:
            index1 = i
    if index1 == 0:
        index2 =  1
    else:
        index2 = 0

    for i in range(n):
        if i != index1 and an[i] > an[index2]:
            index2 = i

    return an[index1] * an[index2]


if __name__ == '__main__':
    try:
        n = int(input())
        a = [int(x) for x in input().split()]
    except EOFError:
        pass
    assert (len(a) == n)

print(max_pairwise_product_fast(n, a))
