import qualified Data.List as List
import qualified Data.Set as Set

numDigits :: (Integral a) => a -> a
numDigits 0 = 1
numDigits n = floor (logBase 10.0 $ fromIntegral n) + 1

digits :: (Integral a) => a -> [a]
digits n = go n [] where
    go 0 [] = [0]
    go 0 acc = List.sort acc
    go n acc = go (n `div` 10) $! (n `mod` 10) : acc

multiples :: (Integral a) => a -> [a]
multiples n =
    let digitCount = numDigits n
    in filter (\x -> numDigits x == digitCount) [n,2*n,3*n,4*n,5*n,6*n]

elementsIdentical :: (Integral a) => [a] -> Bool
elementsIdentical xs = Set.size (Set.fromList xs) <= 1

main = print $ fromList [1,2,3,4,5,6]