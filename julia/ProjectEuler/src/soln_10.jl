# This function implements the sub-linear algorithm from:
# https://projecteuler.net/thread=10;page=5#111677
# AND
# https://gbroxey.github.io/blog/2023/04/09/lucy-fenwick.html
function primesum(n::Integer)
    root = floor(Integer, sqrt(n))
    v = [div(n, x) for x in (1:root)]
    s = Dict([])

    for x in reverse!(collect(1:v[end]))
        push!(v, x)
    end

    for i in v
        s[i] = div(i * (i + 1), 2) - 1
    end

    for p in 2:root
        if s[p] > s[p - 1]
            for x in v
                x < p^2 && break

                y = s[div(x, p)]
                s[x] = s[x] - p * (y - s[p - 1])
            end
        end
    end

    return s[n]
end

println(primesum(2_000_000))