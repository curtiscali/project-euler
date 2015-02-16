#!/usr/bin/python3

from heapq import *

def read_matrix(filename):
    pass

def get_neighbors(row, col, matrix):
    return (matrix[row + 1][col], matrix[row][col + 1])

def get_shortest_dist_vertex(nodes):
    shortest_dist = float("inf")
    shortest = (0, 0)

    for(node, dist)

if __name__ == '__main__':
    sample_matrix = [
        [131, 673, 234, 103, 18],
        [201, 96, 342, 965, 150],
        [630, 803, 746, 422, 111],
        [537, 699, 497, 121, 956],
        [805, 732, 524, 37, 331]
    ]

    nodes = {}
    dist = {}
    previous = {}

    src = (0, 0)
    dest = (4, 4)

    for (value_r, row) in enumerate(sample_matrix):
        for (value_c, col) in enumerate(row):
            nodes[(value_r, value_c)] = col
            dist[(value_r, value_c)] = float("inf")
            previous[(value_r, value_c)] = None

    dist[src] = 0
