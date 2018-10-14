import functools


@functools.lru_cache()
def fib(n):
    if n < 2:
        return n
    else:
        return fib(n-2) + fib(n-1)


for x in range(0, 84):
    print("result [{}]: {}".format(x, fib(x)))