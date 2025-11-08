import sys
sys.path.append('..')

from my_collections.segtree import SegmentTree


def my_sum(*args):
    if not isinstance(args[0], int):
        args = args[0]
    return sum(args)


def varify(func):
    nums = list(range(10))
    n = len(nums)
    seg = SegmentTree(nums, func)

    assert seg.query(0, 9) == func(nums)
    seg.update(0, 100)
    nums[0] = 100
    assert seg.query(0, 9) == func(nums[0:10])
    assert seg.query(0, 0) == func(nums[0:1])
    assert seg.query(3, 5) == func(nums[3:6])

    try:
        seg.update(10, 10)
    except AssertionError:
        pass


def test_seg():
    for f in [min, my_sum, max]:
        varify(f)
