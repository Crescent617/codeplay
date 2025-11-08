from dictionary import Dictionary

def test_dict():
    alphabet = Dictionary(num_buckets=1)
    alphabet.set('a', 'A')
    assert alphabet.get('a') is 'A'

    alphabet.set('b', 'B')
    alphabet.set('c', 'C')
    alphabet.delete('b')
    assert alphabet.get('b') is None
    alphabet.delete('c')
    assert alphabet.get('c') is None
    alphabet.list()
