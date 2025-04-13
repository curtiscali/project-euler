include("number_theory.jl")

function sum_divisibleby_under(target::Integer, n::Integer)
    p = div(target, n)
    n * linearsum(p)
end

N = 1000

println(
    sum_divisibleby_under(N - 1, 3) + sum_divisibleby_under(N - 1, 5) - sum_divisibleby_under(N - 1, 15)
)