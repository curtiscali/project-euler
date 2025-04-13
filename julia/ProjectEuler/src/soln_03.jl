include("primes.jl")

N = 600_851_475_143

primes = primes_below(floor(Integer, sqrt(N)))

for p in reverse!(primes)
    if N % p == 0
        println(p)
        break
    end
end