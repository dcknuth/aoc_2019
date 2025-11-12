import time
from functools import wraps

def time_it(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        T0 = time.perf_counter()
        result = func(*args, **kwargs)
        T1 = time.perf_counter()
        # only seems to go to 7 decimals, then all 0s
        print(f"{func.__name__} took {T1 - T0:.7f} seconds")
        return result
    return wrapper