#!/usr/bin/env ruby

def num_digits(n)
   return Math.log10(n).floor + 1 
end

if __FILE__ == $0
    sum = 0

    IO.readlines("data.txt").each do |line|
        sum += line.to_i
    end

    divisor = num_digits(sum) - 10

    puts "#{sum / 10**divisor}"
end