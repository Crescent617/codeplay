from dllist import DoubleLinkedList

class Dictionary(object):
    def __init__(self, num_buckets=256):
        """Initializes a Map with the given number of buckets. 更多的buckets的目的是在于加速get_slot搜索过程！！！"""
        self.map = DoubleLinkedList()
        self.num_buckets = num_buckets
        for i in range(0, self.num_buckets):
            self.map.push(DoubleLinkedList())

    def get_bucket(self, key):
        """Given a key, find the bucket where it would go."""
        bucket_id = hash(key) % self.num_buckets
        return self.map.get(bucket_id)

    def get_slot(self, key, default=None):
        """
        Returns either the bucket and node for a slot, or None, None
        """
        bucket = self.get_bucket(key)

        if bucket:
            node = bucket.begin
            while node:
                if key == node.value[0]:
                    return bucket, node
                else:
                    node = node.next

        # fall through for both if and while above
        return bucket, None

    def get(self, key, default=None):
        """Gets the value in a bucket for the given key, or the default."""
        bucket, node = self.get_slot(key, default=default)
        return node and node.value[1] or node

    def set(self, key, value):
        """Sets the key to the value, replacing any existing value."""
        bucket, slot = self.get_slot(key)
        if slot:
            # the key exists, replace it
            slot.value = (key, value)
        else:
            # the key does not, append to create it
            bucket.push((key, value))

    def delete(self, key):
        """Deletes the given key from the Map."""
        bucket, node = self.get_slot(key)
        bucket.detach(node)

    def list(self):
        """Prints out what's in the Map."""
        bucket_node = self.map.begin
        while bucket_node:
            slot_node = bucket_node.value.begin
            while slot_node:
                print(slot_node.value)
                slot_node = slot_node.next
            bucket_node = bucket_node.next


