from sorting import *
from dllist import DoubleLinkedList
from random import randint
from cProfile import Profile

max_num = 5000

def random_list(count):
    numbers = DoubleLinkedList()
    for i in range(0, count):
        numbers.shift(randint(0, 10000))
    return numbers

def is_sorted(numbers):
    node = numbers.begin.next
    while node:
        if node.prev.value > node.value:
            return False
        else:
            node = node.next
    return True

def test_bubble():
    numbers = random_list(max_num)
    bubble_sort(numbers)

    assert is_sorted(numbers)

def test_merge():
    numbers = random_list(max_num)
    merge_sort(numbers)

    assert is_sorted(numbers)

def test_quick():
    numbers = [randint(0, 10000) for i in range(max_num)]
    quick_sort(numbers, 0, max_num-1)
    i = 1
    while i < max_num:
        assert numbers[i] >= numbers[i-1]
        i += 1

def test_all():
    numbers = [randint(0, 10000) for i in range(max_num)]
    numbers_m = DoubleLinkedList()
    numbers_b = DoubleLinkedList()
    for i in numbers:
        numbers_m.shift(i)
        numbers_b.shift(i)
    quick_sort(numbers, 0, max_num-1)
    merge_sort(numbers_m)
    bubble_sort(numbers_b)


if __name__ == '__main__':
    prof = Profile()
    prof.enable()
    test_all()
    prof.create_stats()
    prof.print_stats('sorting.py', sort="cumulative")


    