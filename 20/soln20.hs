import Data.Char

factorial :: (Integral a) => a -> a
factorial n
    | n < 0 = 1
    | otherwise = foldr (*) 1 [1..n]

digitSum :: (Show a) => a -> Int
digitSum n = foldr (+) 0 (map (digitToInt) (show n))

main = print $ digitSum (factorial 100)