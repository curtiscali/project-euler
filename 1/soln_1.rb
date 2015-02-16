#!/usr/bin/ruby

if __FILE__ == $0
    i = 0
    sum = 0

    while i < 1000 do
        sum += i
        i += 3
    end

    i = 0
    while i < 1000 do
        sum += i
        i += 5
    end

    i = 0
    while i < 1000 do
        sum -= i
        i += 15
    end

    puts(sum)
end
