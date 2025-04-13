function _multiplier(x::Integer, y::Integer)
    if x == y
        return x
    end

    multiplier = 1
    for i in x:y
        multiplier *= i
    end

    return multiplier
end

function combinations(n::Integer, r::Integer)
    if n < r
        error("n must be greater than or equal to r")
    elseif n == r || r == 0
        return 1
    end

    maximum = max(r, n - r)
    minimum = min(r, n - r)

    numerator = _multiplier(maximum + 1, n)
    denominator = _multiplier(1, minimum)

    return div(numerator, denominator)
end