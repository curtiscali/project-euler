include("number_theory.jl")
include("primes.jl")

function numfactors(n::Integer)
    prod([v + 1 for (k, v) in prime_factors(n)])
end

const TARGET_NUM_DIVISORS::Integer = 500

n = 7
while numfactors(linearsum(n)) <= TARGET_NUM_DIVISORS
    global n += 1
end

println(linearsum(n))