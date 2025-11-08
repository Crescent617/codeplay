from queue import *

def test_shift():
    animals = Queue()
    animals.shift('cat')
    assert animals.first.value is 'cat'
    animals.shift('dog')
    assert animals.first.value is 'cat'
    assert animals.last.value is 'dog'

def test_unshift():
    animals = Queue()
    animals.shift('cat')
    assert animals.unshift() is 'cat'
    assert animals.first is None
    animals.shift('cat')
    animals.shift('dog')
    assert animals.unshift() is 'cat'
    assert animals.first is animals.last
    