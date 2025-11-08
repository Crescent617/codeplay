class FenwickTree:

    def __init__(self, nums):
        self.arr = [0] + nums
        n = len(self.arr)
        for i in range(1, n):
            j = i + (i & -i)
            if j < n:
                self.arr[j] += self.arr[i]

    def update(self, i, delta):
        i += 1
        assert 0 < i < len(self.arr)

        while i < len(self.arr):
            self.arr[i] += delta
            i += i & -i

    def prefix(self, i):
        res = 0
        while i > 0:
            res += self.arr[i]
            i -= i & -i
        return res

    def __repr__(self):
        return self.arr.__repr__()
