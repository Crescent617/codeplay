class BSTreeNode(object):

    def __init__(self, key, value, left=None, right=None, parent=None):
        self.key = key
        self.value = value
        self.left = left
        self.right = right
        self.parent = parent

    def find_minimum(self):
        node = self
        while node.left:
            node = node.left
        return node

    def exchange(self, node):
        self.key, self.value, node.key, node.value = (
            node.key, node.value, self.key, self.value)

    def replace_node(self, node):
        """Given a child, it will find the child, move it's value to here,
         then remove it.
        Copied from wikipedia to solve it quicker.
        """
        if self.parent:
            if self == self.parent.left:
                self.parent.left = node
            else:
                self.parent.right = node
        if node:
            node.parent = self.parent

    def __repr__(self):
        return f"{self.key}={self.value}: \n{self.left}<-- -->{self.right}"


class BSTree(object):

    def __init__(self):
        self.root = None

    def __getitem__(self, key):
        return self.get(key)

    def __setitem__(self, key, val):
        return self.set(key, val)

    def get(self, key):
        node = self.root
        while node:
            if key == node.key:
                return node.value
            elif key < node.key:
                node = node.left
            elif key > node.key:
                node = node.right
            else:
                assert False, 'This is impossible.'
        return None

    def set(self, key, value):
        if not self.root:
            self.root = BSTreeNode(key, value)

        node = self.root
        while node:
            if key == node.key:
                node.value = value
                break
            elif key < node.key:
                if not node.left:
                    node.left = BSTreeNode(key, value, parent=node)
                    break
                node = node.left
            elif key > node.key:
                if not node.right:
                    node.right = BSTreeNode(key, value, parent=node)
                    break
                node = node.right
            else:
                assert False

    def _delete(self, key, node):
        while node:
            if key == node.key:
                if node.left and node.right:
                    successor = node.right.find_minimum()
                    node.exchange(successor)
                    self._delete(key, successor)
                elif node.left:
                    if node == self.root:
                        self.root = node.left
                    node.replace_node(node.left)
                    break
                elif node.right:
                    if node == self.root:
                        self.root = node.right
                    node.replace_node(node.right)
                    break
                else:
                    node.replace_node(None)
                    break
            elif key < node.key:
                node = node.left
            elif key > node.key:
                node = node.right
            else:
                assert False

    def delete(self, key):
        if self.root:
            self._delete(key, self.root)

    def _show(self, node, indent=0):
        """List the elements in the tree."""
        assert node, "Invalid node given."

        if node:
            print(node.key, "=", node.value)

            if node.left:
                print(" " * indent, "<- ", end="")
                self._show(node.left, indent+2)
            if node.right:
                print(" " * indent, "-> ", end="")
                self._show(node.right, indent+2)

    def show(self, start=""):
        print("\n\n----", start)
        self._show(self.root)
