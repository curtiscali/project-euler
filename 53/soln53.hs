import System.IO

nCr :: Int -> Int -> Integer
nCr n r = round $ nCr' n r 1.0 where
    nCr' x y acc
        | y == 0 = acc
        | x == y = acc
        | otherwise = nCr' (x - 1) (y - 1) $!
            ((fromIntegral x / fromIntegral y) * acc)

main = do
    let allCombos = [nCr n r | n <- [23..100], r <- [10..100]]
        numQualified = length $ filter (\x -> x <= 1000000) allCombos
    print numQualified