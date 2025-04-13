function linearsum(n::Integer)
    div((n * (n + 1)), 2)
end

function quadraticsum(n::Integer)
    div((n^2 + n) * (2n + 1), 6)
end

function reverse(n::Integer, base::Integer = 10)
    reversed = 0

    i = n
    while i > 0
        reversed *= base;
        reversed += (i % base)

        i = div(i, base)
    end

    return reversed
end

function ispalindrome(n::Integer, base::Integer = 10)
    return n == reverse(n, base)
end