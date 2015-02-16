import System.IO

nCr :: Int -> Int -> Integer
nCr n r = round $ nCr' n r 1.0 where
    nCr' x y acc
        | y == 0 = acc
        | x == y = acc
        | otherwise = nCr' (x - 1) (y - 1) $!
            ((fromIntegral x / fromIntegral y) * acc)

pascal :: Int -> [Integer]
pascal n = map (\x -> nCr n x) [0..n]

numQualified :: Int -> Int
numQualified n = length (filter (\x -> x `mod` 7 /= 0) row)
    where row = pascal n

map' :: (a -> b) -> [a] -> [b]
map' f xs = foldr (\x acc -> f x : acc) [] xs  

main = do
    let totalQualified = sum $ map' numQualified [7..1000000]
    print totalQualified