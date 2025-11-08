import math


class SparseTable:
    def __init__(self, nums: list) -> None:
        func = min
        n = len(nums)
        deg = 0
        while 1 << deg < n:
            deg += 1

        st = [[float('inf')] * (deg + 1) for _ in range(n)]

        for i in range(n):
            st[i][0] = nums[i]

        for j in range(1, deg + 1):
            for i in range(n):
                if i + 2 ** (j - 1) >= n:
                    break
                st[i][j] = func(st[i][j - 1], st[i + 2 ** (j - 1)][j - 1])

        self.st = st
        self.func = func

    def query(self, left, right, includeRight=True):
        deg = math.floor(math.log2(right - left + includeRight))
        return self.func(self.st[left][deg], self.st[right - 2 ** deg + 1][deg])


if __name__ == "__main__":

    st = SparseTable(list(range(10)))
    assert st.query(0, 9) == 0
    assert st.query(2, 5) == 2
    assert st.query(8, 9) == 8
