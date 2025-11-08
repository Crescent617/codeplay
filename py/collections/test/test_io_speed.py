import sys
import time
# import line_profiler

read_buf1 = sys.stdin.readline

def read_buf():
    return sys.stdin.buffer.readline()

a = 0
print(a)
# def test_input(f):
#     start = time.time()
#     while f():
#         pass
#     print('{} spend {}'.format(f.__name__, time.time()-start))


# if __name__ == "__main__":
#     read_buf()
