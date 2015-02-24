selfPowerSum :: (Integral a) => a
selfPowerSum = go 1 0 where
    go 1000 acc = acc
    go n acc = go (n + 1) $! (n^n + acc)

main = print $ selfPowerSum `mod` 10000000000