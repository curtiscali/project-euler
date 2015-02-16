from time import clock

def find_kl(permutation):
    k = None
    for i in range(len(permutation) - 1):
        if permutation[i] < permutation[i+1]:
            k = i
    if k is None:
        return None
    else:
        l = None
        for (i, value) in enumerate(permutation):
            if value > permutation[k]:
                l = i
        return (k, l)

def reverse_segment(permutation, start):
    sublist = permutation[start:]
    sublist.reverse()

    return permutation[0:start] + sublist

def swap(ls, a, b):
    ls[a], ls[b] = ls[b], ls[a]

def create_successor(start):
    indices = find_kl(start)

    if indices is None:
        return start
    else:
        k, l = indices
        swap(start, k, l)
        start = reverse_segment(start, k+1)
        return start

def main():
    start = [0,1,2,3,4,5,6,7,8,9]
    current = start
    numPermutations = 1

    start = clock()
    while numPermutations < 1000000:
        current = create_successor(current)
        numPermutations += 1
    end = clock()

    print(end - start, "s")
    print(current)

if __name__ == '__main__':
    main()