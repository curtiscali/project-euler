include("primes.jl")

N = 10001
upper_bound = floor(Integer, (N * log(N)) + (N * log(log(N))))
primes = primes_below(upper_bound)

println(primes[N])