#!/usr/bin/python3.3

def dot_product(v1, v2):
    return (v1[0] * v2[0]) + (v1[1] * v2[1])

def vec_sub(v1, v2):
    return (v1[0] - v2[0], v1[1] - v2[1])

def triangle_contains(triangle, p):
    a,b,c = triangle[0],triangle[1],triangle[2]

    v1,v2,v3 = vec_sub(c, a),vec_sub(b, a),vec_sub(p, a)

    dot11 = dot_product(v1, v1)
    dot12 = dot_product(v1, v2)
    dot13 = dot_product(v1, v3)
    dot22 = dot_product(v2, v2)
    dot23 = dot_product(v2, v3)
    
    invDenom = 1 / (dot11 * dot22 - dot12 * dot12)
    u = (dot22 * dot13 - dot12 * dot23) * invDenom
    v = (dot11 * dot23 - dot12 * dot13) * invDenom
    
    return (u >= 0) and (v >= 0) and (u + v < 1)

def mk_triangle(coordinates):
    return (
        (coordinates[0], coordinates[1]),
        (coordinates[2], coordinates[3]),
        (coordinates[4], coordinates[5])
    )


ORIGIN = (0, 0)
triangles = (
    mk_triangle([int(x) for x in line.split(",")]) for line in
    open("triangles.txt")
)
num_with_origin = sum(
    (1 for triangle in triangles if triangle_contains(triangle, ORIGIN))
)

print(num_with_origin)
