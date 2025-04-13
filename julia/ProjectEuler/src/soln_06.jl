include("number_theory.jl")

N = 100
lsum = linearsum(N)
qsum = quadraticsum(N)

println(lsum^2 - qsum)