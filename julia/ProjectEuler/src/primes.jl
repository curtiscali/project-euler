function primes_below(n::Integer)
    @inbounds begin
        primes = [2, 3]
        
        if n < 0
            error("n can't be negative (found $(n))")
        elseif n < 2
            primes = Integer[]
        elseif n == 2
            primes = [2]
        else
            factor = floor(Integer, sqrt(n))
            sieve = falses(n)
        
            for x = 1:factor
                for y = 1:factor
                    m = 4x^2 + y^2
                    if m <= n && (m % 12 == 1 || m % 12 == 5)
                        sieve[m] = !sieve[m]
                    end
            
                    m = 3x^2 + y^2
                    if m <= n && m % 12 == 7
                        sieve[m] = !sieve[m]
                    end
            
                    m = 3x^2 - y^2
                    if x > y && m <= n && m % 12 == 11
                        sieve[m] = !sieve[m]
                    end
                end
            end
        
            for x = 5:factor
                if sieve[x]
                    for y = x^2 : x^2 : n
                        sieve[y] = false
                    end
                end
            end
        
            for i = 5:n
                if sieve[i]
                    push!(primes, i)
                end
            end
        end
    end

    return primes
end

function prime_factors(number::Integer)
    factors = Dict{Integer, Integer}([])
    n = number

    while n % 2 == 0
        if haskey(factors, 2)
            factors[2] = factors[2] + 1
        else
            factors[2] = 1
        end

        n = div(n, 2)
    end

    i = 3
    while i * i <= n
        while n % i == 0
            if haskey(factors, i)
                factors[i] = factors[i] + 1
            else
                factors[i] = 1
            end

            n = div(n, i)
        end

        i += 2
    end

    if n > 2
        if haskey(factors, n)
            factors[n] = factors[n] + 1
        else
            factors[n] = 1
        end
    end

    return factors
end