class StackNode:

    def __init__(self, value, nxt):
        self.value = value
        self.next = nxt
    
    def __repr__(self):
        nval = self.next and self.next.value or None
        return f'[{self.value}:{repr(nval)}]'

class Stack:

    def __init__(self):
        self.top = None
    
    def push(self, obj):
        if self.top:
            node = StackNode(obj, self.top)
            self.top = node
        else:
            self.top = StackNode(obj, None)
    
    def pop(self):
        if self.top:
            temp = self.top.value
            self.top = self.top.next
            return temp
        else:
            return None
    
    def first(self):
        return self.top and self.top.value or None

    def count(self):
        count = 0
        node = self.top

        while node:
            count += 1
            node = node.next
        return count
    
    def dump(self, mark = '------'):
        node = self.top

        print(mark)
        while node:
            print(node.value, end=' ')
            node = node.next
        print(mark)
        return 0

