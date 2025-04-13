include("number_theory.jl")

largest_product = 0
for i in 100:999
    for j in 100:999
        product = i * j
        if ispalindrome(product) && product > largest_product
            global largest_product = product
        end
    end
end

println(largest_product)