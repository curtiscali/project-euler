function collatz(n::Int64, solutions::Dict{Int64, Int64})
    m = n
    length = 1

    while m > 1
        if m % 2 == 0
            half = m / 2
            if haskey(solutions, half)
                length += solutions[half]
                break
            else
                m /= 2
            end
        else
            mult = 3m + 1
            if haskey(solutions, mult)
                length += solutions[mult]
                break
            else
                m = mult
            end
        end

        length += 1
    end

    solutions[n] = length
    return length
end

const LIMIT::Int64 = 1_000_000

solutions::Dict{Int64, Int64} = Dict([])
longest_path = 1
longest_path_num = 1

for i in div(LIMIT, 2):LIMIT
    chain_length = collatz(i, solutions)
    if chain_length > longest_path
        global longest_path = chain_length
        global longest_path_num = i
    end
end

println(longest_path_num)