class Node:
    def __init__(self, val, next = None, prev = None) -> None:
        self.val = val
        self.next = next
        self.prev = prev

class LinkedList:
    def __init__(self) -> None:
        self.tail = Node("Dummy")
        self.len = 0

    def add(self, val):
        node = Node(val)

        node.prev = self.tail
        self.tail.next = node
        self.tail = node

    def show(self):
        cur = self.tail

        while cur:
            print(f"{cur.val}", end = "->")
            cur = cur.prev

l = LinkedList()

l.add("nd")
l.add("nf python")
l.add("test")

l.show()
