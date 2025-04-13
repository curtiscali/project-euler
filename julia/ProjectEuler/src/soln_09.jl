function factor_pairs(n::Int)
    root = floor(Int, sqrt(n))

    factor_pairs = []
    
    for i in 1:root
        if n % i == 0
            push!(factor_pairs, (i, div(n, i)))
        end
    end

    return factor_pairs
end

function pythag_triple(m::Int, n::Int)
    (abs(m^2 - n^2), 2 * m * n, m^2 + n^2)
end

TARGET_SUM = 1000

found_triple = false
for i in (div(TARGET_SUM, 10):2:div(TARGET_SUM, 2))
    for (x, y) in factor_pairs(div(i, 2))
        (a, b, c) = pythag_triple(x, y)

        if a + b + c == TARGET_SUM
            println(a * b * c)
            global found_triple = true
        end

        found_triple && break
    end
end