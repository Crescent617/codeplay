from stack import *

def test_push():
    animals = Stack()
    animals.push('cat')
    assert animals.top.value is 'cat'
    animals.push('dog')
    assert animals.top.value is 'dog'

def test_pop():
    animals = Stack()
    animals.push('cat')
    animals.push('dog')
    assert animals.pop() is 'dog'
    assert animals.pop() is 'cat'

def test_count():
    animals = Stack()
    assert animals.count() == 0
    animals.push('cat')
    animals.push('dog')
    assert animals.count() == 2