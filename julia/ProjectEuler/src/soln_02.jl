MAX_FIB = 4_000_000

global sum = 0
global fib1 = 0
global fib2 = 1

while fib2 <= MAX_FIB
    local new_fib = fib1 + fib2
    global fib1 = fib2
    global fib2 = new_fib

    if iseven(fib2)
        global sum += fib2
    end
end

println(sum)