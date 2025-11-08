class QueueNode:

    def __init__(self, value, prev, nxt):
        self.value = value
        self.prev = prev
        self.next = nxt
    
    def __repr__(self):
        nval = self.next and self.next.value or None
        pval = self.prev and self.prev.value or None
        return f"[{self.value}, {repr(nval)}, {repr(pval)}]"
    

class Queue:

    def __init__(self):
        self.first = None
        self.last = None

    def shift(self, val):
        if self.last:
            node = QueueNode(val, self.last, None)
            self.last.next = node
            self.last = node
            assert self.first is not self.last
        else:
            assert self.first is None
            node = QueueNode(val, None, None)
            self.first = self.last = node

    def unshift(self):
        if self.first:
            temp = self.first.value
            if self.first is self.last:
                self.first = self.last = None
                return temp
            self.first = self.first.next
            self.first.prev = None
            return temp
        else:
            return None
    
    def count(self):
        count = 0
        node = self.first
        while node:
            count += 1
            node = node.next
        return count
    
    def dump(self):
        node = self.first
        while node:
            print(node.value, end=' ')
            node = node.next
        

            

