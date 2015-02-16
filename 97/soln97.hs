main = do
    let number = 28433 * (2^7830457) + 1
    print $! number `mod` 10000000000
