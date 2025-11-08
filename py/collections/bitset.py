from array import array


class Bitset:

    def __init__(self, n, unit=8):
        self.unit = unit
        num = (n + unit - 1) // unit
        self.data = array('B', [0]*num)

    def __getitem__(self, i):
        """
        >>> bs = Bitset(10); bs.set(9); bs[9]
        1
        """
        idx, pos = i // 8, i % 8
        return (self.data[idx] >> pos) & 1

    def _set_val(self, i, val):
        idx, pos = i // self.unit, i % self.unit
        mask = 1 << pos
        if val == 1:
            self.data[idx] |= mask
        else:
            self.data[idx] &= ~mask

    def __len__(self):
        return len(self.data) * self.unit

    def set(self, i):
        self._set_val(i, 1)

    def reset(self, i):
        self._set_val(i, 0)

    def __ixor__(self, other):
        """
        >>> bs1 = Bitset(8)
        >>> for i in range(0, 8, 2):
        ...     bs1.set(i)
        >>> bs2 = Bitset(8)
        >>> for i in range(1, 8, 2):
        ...     bs2.set(i)
        >>> bs1 ^= bs2; bs1
        11111111
        """
        for i, d in enumerate(other.data):
            self.data[i] ^= d
        return self

    def __repr__(self):
        """
        >>> bs = Bitset(12); bs.set(10); bs
        0000000000100000
        """
        return ''.join(bin(b)[2:].zfill(self.unit)
                       for b in reversed(self.data))[::-1]