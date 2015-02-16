numDigits :: Int -> Int
numDigits n = go n 0 where
    go 0 acc = acc
    go num acc = go (num `div` 10) $! 1 + acc

digits :: Int -> [Int]
digits n = go n [] where
    go 0 acc = acc
    go num acc = go (num `div` 10) $! (num `mod` 10):acc 

reverseInt :: Int -> Int
reverseInt n = go n 0 where
    go 0 acc = acc
    go n acc = go (n `div` 10) $! ((acc * 10) + n `mod` 10)

isReversible :: Int -> Bool
isReversible 0 = True
isReversible n = length digitList == numDigits sum where
    sum = n + reverseInt n
    digitList = filter odd $ digits sum

numReversibles :: Int -> Int
numReversibles limit = go 10001 (limit - 1) 0 where
    go curr lim acc
        | curr >= lim = acc
        | curr `mod` 10 == 0 = go (curr + 1) lim acc
        | isReversible curr = go (curr + 1) lim $! acc + 1
        | otherwise = go (curr + 1) lim acc

main = print $ 120 + numReversibles 10000000000
