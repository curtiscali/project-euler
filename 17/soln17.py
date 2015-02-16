to_nineteen = [
    "", "one", "two",  "three", "four", "five", "six", "seven", "eight", "nine",
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
    "seventeen", "eighteen", "nineteen"
]

tens = [
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
]

LENGTH_OF_ONE_THOUSAND = 11
LENGTH_OF_HUNDRED = 7
LENGTH_OF_AND = 3

def tens_length(n):
    tens_digit = n // 10
    ones_digit = n % 10
    return len(tens[tens_digit - 2]) + len(to_nineteen[ones_digit])

def main():
    total_chars = 0

    for i in range(1, 1001):
        if i < 20:
            total_chars += len(to_nineteen[i])
        elif i < 100:
            total_chars += tens_length(i)
        elif i < 1000:
            hundreds_digit = i // 100
            total_chars += len(to_nineteen[hundreds_digit]) + LENGTH_OF_HUNDRED

            tens_part = i - (hundreds_digit*100)
            if tens_part < 20:
                total_chars += len(to_nineteen[tens_part])
                if tens_part != 0: total_chars += LENGTH_OF_AND
            else:
                total_chars += tens_length(tens_part) + LENGTH_OF_AND
        else:
            total_chars += LENGTH_OF_ONE_THOUSAND

    print(total_chars)

if __name__ == '__main__':
    main()