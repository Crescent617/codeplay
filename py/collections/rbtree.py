_RED = False
_BLACK = True


class RBTNode:

    def __init__(self, key, val=None, color=_RED, parent=None):
        self.key = key
        self.val = val
        self.left = None
        self.right = None
        self.parent = parent
        self.color = color

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
        """
        assert node in (self.left, self.right)

        if self.parent:
            if self == self.parent.left:
                self.parent.left = node
            else:
                self.parent.right = node
        if node:
            node.parent = self.parent

    def left_rotate(self):
        """
        >>> n1 = RBTNode(0);
        >>> n2 = RBTNode(1, parent=n1); n3 = RBTNode(2, parent=n2);
        >>> n1.right = n2; n2.right = n3; n2.left = RBTNode(3, parent=n2)
        >>> n1.left_rotate()
        >>> n1.parent.key
        1
        >>> n1.right.key
        3
        >>> n2.left.key
        0
        """
        nxt_par = self.right
        assert self.right

        grandpa = self.parent
        self.parent = nxt_par
        nxt_par.parent = grandpa

        self.right = nxt_par.left
        nxt_par.left = self

        if grandpa:
            is_left = True if grandpa.left is self else False
            if is_left:
                grandpa.left = nxt_par
            else:
                grandpa.right = nxt_par

    def right_rotate(self):
        """
        >>> n2 = RBTNode(1);
        >>> n1 = RBTNode(0, parent=n2); n3 = RBTNode(2, parent=n2);
        >>> n2.left = n1; n2.right = n3;
        >>> n2.right_rotate()
        >>> n1.parent
        >>> n2.parent.key
        0
        >>> n1.right.key
        1
        """
        nxt_par = self.left
        assert self.left

        grandpa = self.parent
        self.parent = nxt_par
        nxt_par.parent = grandpa

        self.left = nxt_par.right
        nxt_par.right = self

        if grandpa:
            is_left = True if grandpa.left is self else False
            if is_left:
                grandpa.left = nxt_par
            else:
                grandpa.right = nxt_par

    def __repr__(self):
        return (f"{self.parent.key if self.parent else None} -->"
                f" {self.key}({self.color})\n{self.left}<-- -->{self.right}")


class RedBlackTree:

    def __init__(self):
        self.root = None

    def __getitem__(self, key):
        return self.get(key)

    def __setitem__(self, key, val):
        return self.add(key, val)

    def __contains__(self, key):
        return bool(self._get(key))

    def get(self, key):
        node = self._get(key)
        return node.val if node else None

    def _get(self, key):
        node = self.root
        while node:
            if key == node.key:
                return node
            elif key < node.key:
                node = node.left
            elif key > node.key:
                node = node.right
            else:
                assert False, 'This is impossible.'
        return None

    def add(self, key, val=None):
        if not self.root:
            self.root = RBTNode(key, val, color=_BLACK)
            return

        node = self.root
        added = False
        while node and not added:
            # in this case just return
            if key == node.key:
                node.val = val
                break
            # in other cases fixup is needed
            elif key < node.key:
                if not node.left:
                    node.left = RBTNode(key, val, parent=node)
                    added = True
                node = node.left
            elif key > node.key:
                if not node.right:
                    node.right = RBTNode(key, val, parent=node)
                    added = True
                node = node.right
            else:
                assert False

        if added:
            self.add_fixup(node)
        # self.show()
        return

    def add_fixup(self, node: RBTNode):
        while node and node.parent and node.parent.color == _RED:
            self.show()
            par = node.parent
            grandpa = par.parent

            if par is grandpa.left:
                uncle = grandpa.right
                is_node_left = True if par.left is node else False

                if uncle and uncle.color == _RED:
                    par.color = _BLACK
                    uncle.color = _BLACK
                    grandpa.color = _RED
                    node = grandpa
                elif not is_node_left:
                    node = par
                    node.left_rotate()
                # the left left case
                elif is_node_left:
                    par.color = _BLACK
                    grandpa.color = _RED
                    grandpa.right_rotate()
            else:
                uncle = grandpa.left
                is_node_right = True if par.right is node else False

                if uncle and uncle.color == _RED:
                    par.color = _BLACK
                    uncle.color = _BLACK
                    grandpa.color = _RED
                    node = grandpa
                elif not is_node_right:
                    node = par
                    node.right_rotate()
                # the right right case
                elif is_node_right:
                    par.color = _BLACK
                    grandpa.color = _RED
                    grandpa.left_rotate()

        while self.root.parent:
            self.root = self.root.parent

        self.root.color = _BLACK

    def _delete(self, key, node):
        deleted = None
        replaced = None
        while node:
            if key == node.key:
                deleted = node
                if node.left and node.right:
                    successor = node.right.find_minimum()
                    node.exchange(successor)
                    # recur
                    deleted = self._delete(key, successor)
                elif node.left:
                    if node == self.root:
                        self.root = node.left
                    replaced = node.left
                    node.replace_node(node.left)
                    break
                elif node.right:
                    if node == self.root:
                        self.root = node.right
                    replaced = node.right
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

        if deleted and deleted.color == _BLACK:
            self.del_fixup(replaced)
        return deleted

    def delete(self, key):
        self._delete(key, self.root)

    def del_fixup(self, node):
        if node is self.root or node.color == _RED:
            node.color = _BLACK
            return
        # TODO: finish this method

    def _show(self, node, indent=0):
        """List the elements in the tree."""
        assert node, "Invalid node given."

        clr = {True: 'black', False: 'red'}

        if node:
            print(f'{node.key} ({clr[node.color]})')
            if node.left:
                print(" " * indent, "<- ", end="")
                self._show(node.left, indent+2)
            if node.right:
                print(" " * indent, "-> ", end="")
                self._show(node.right, indent+2)

    def show(self, start=""):
        print("\n\n----", start)
        self._show(self.root)
