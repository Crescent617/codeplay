
class Stream:

    def __init__(self, first, compute_rest, empty=False):
        self.first = first
        self._compute_rest = compute_rest
        self._rest = None
        self._calculated = False
        self.empty = empty

    @property
    def rest(self):
        assert not self.empty
        if not self._calculated:
            self._calculated = True
            self._rest = self._compute_rest()
        return self._rest


def int_stream(first=1) -> Stream:
    def compute_rest(): 
        return int_stream(first+1)
    return Stream(first, compute_rest)


def map_stream(f, s: Stream) -> Stream:
    if s.empty:
        return s
    def compute_rest(): 
        return map_stream(f, s.rest)
    return Stream(f(s.first), compute_rest)


def filter_stream(f, s: Stream):
    if s.empty:
        return s if f(s.first) else None

    def compute_rest():
        return filter_stream(f, s.rest)
        
    if f(s.first):
        return Stream(s.first, compute_rest)
    return compute_rest()


def truncate_stream(s: Stream, n):
    if n == 0 or s.empty:
        s.empty = True
        return

    def compute_rest():
        return truncate_stream(s.rest, n-1)

    return Stream(s.first, compute_rest)


def stream_to_list(s: Stream) -> list:
    res = []
    while s:
        res.append(s.first)
        s = s.rest
    return res


def find_prime(s=int_stream(2)) -> Stream:
    def compute_rest():
        return filter_stream(lambda x: x % s.first != 0, find_prime(s.rest))
    return Stream(s.first, compute_rest)


if __name__ == "__main__":
    import sys

    sys.setrecursionlimit(10**5)
    p = find_prime()
    for i in range(100):
        print(p.first)
        p = p.rest
