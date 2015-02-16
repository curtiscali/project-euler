import System.IO

factorPairs :: (Integral a) => a -> [(a, a)]
factorPairs n = [(x, n `div` x) | x <- [1..floor (sqrt n)], n `mod` x == 0]

isPrime :: Int -> Bool
isPrime n = length (factorPairs n) == 1

main = print $ factorPairs 10