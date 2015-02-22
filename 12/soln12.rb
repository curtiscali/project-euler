#!/usr/bin/env ruby

class Soln12
    def triangle(n)
        return (n * (n + 1)) / 2
    end

    def get_num_factors(n)
        root = Math.sqrt(n).ceil.to_i
        num_factors = 0

        (1..root).each do |i|
            if n % i == 0
                if i == root
                    num_factors += 1
                else
                    num_factors += 2
                end
            end
        end

        return num_factors
    end
end


if __FILE__ == $0
    solution = Soln12.new
    n = 7

    begin
        n += 1
    end until solution.get_num_factors(solution.triangle(n)) > 500

    puts "#{solution.triangle(n)}"
end