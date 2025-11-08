class SegmentTree:
    def __init__(self, items, interval_info=min):
        # assert callable(interval_info)

        raw_size, n = len(items), 1
        while n < raw_size:
            n *= 2

        items = [0] * n + items + [0] * (n - raw_size)
        for i in range(n)[::-1]:
            items[i] = interval_info(items[2 * i], items[2 * i + 1])

        self.interval_info = interval_info
        self.items = items
        self.size = raw_size
        self.offset = n

    def update(self, i, val):
        assert 0 <= i < self.size
        i = self.offset + i
        self.items[i] = val

        func, items = self.interval_info, self.items
        while i > 0:
            i >>= 1
            items[i] = func(items[2 * i], items[2 * i + 1])

    def query(self, left, right, inclusive=True):
        if not inclusive:
            right -= 1

        assert 0 <= left <= right < self.size

        func, items = self.interval_info, self.items
        left += self.offset
        right += self.offset

        # as func has two argument, add left first
        ans = items[left]
        left += 1

        while left < right:
            if left % 2:
                ans = func(items[left], ans)
                left += 1
            if right % 2 == 0:
                ans = func(items[right], ans)
                right -= 1
            left >>= 1
            right >>= 1
        if left == right:
            ans = func(ans, items[left])
        return ans

    def show(self):
        i = 1
        while i <= self.offset:
            print(*self.items[i : 2 * i])
            i *= 2


MOD = 1e9 + 7


class SegmentTreeWithLazyPropagation:
    def __init__(self, sums):
        raw_size, n = len(sums), 1
        while n < raw_size:
            n *= 2

        sums2 = [0] * n + [s ** 2 % MOD for s in sums] + [0] * (n - raw_size)
        sums = [0] * n + sums + [0] * (n - raw_size)

        for i in range(1, n)[::-1]:
            sums[i] = sums[2 * i] + sums[2 * i + 1]
            sums2[i] = sums2[2 * i] + sums2[2 * i + 1]

        self.sums = sums
        self.sums2 = sums2
        self.size = n
        self.lazy = [0] * (2 * n)
        # print('SegmentTree:', self.sums, self.sums2)

    def update_range(self, left, right, diff):
        def _rupdate(cur, lo, hi):
            length = hi - lo + 1

            # add lazy first
            if lazy[cur]:
                lz, pre, pre2 = lazy[cur], sums[cur], sums2[cur]
                sums[cur] = (pre + length * lz) % MOD
                sums2[cur] = (pre2 + 2 * lz * pre + lz ** 2 * length) % MOD
                # propagate to its children
                if lo != hi:
                    lazy[cur * 2] += lz
                    lazy[cur * 2 + 1] += lz
                # reset lazy
                lazy[cur] = 0

            # if completely inside, just do it
            if left <= lo and hi <= right:
                pre, pre2 = sums[cur], sums2[cur]
                sums[cur] = (pre + length * diff) % MOD
                sums2[cur] = (pre2 + 2 * diff * pre + diff ** 2 * length) % MOD
                if lo != hi:
                    lazy[cur * 2] += diff
                    lazy[cur * 2 + 1] += diff

            # if completely outside, pass
            elif lo > right or hi < left:
                pass

            # if intersect, divide and conquer
            else:
                mid = (lo + hi) // 2
                l_sum, l_sum2 = _rupdate(cur * 2, lo, mid)
                r_sum, r_sum2 = _rupdate(cur * 2 + 1, mid + 1, hi)
                sums[cur] = (l_sum + r_sum) % MOD
                sums2[cur] = (l_sum2 + r_sum2) % MOD

            return sums[cur], sums2[cur]

        n, sums, sums2, lazy = self.size, self.sums, self.sums2, self.lazy
        ans = _rupdate(1, 0, n - 1)[1]
        return ans