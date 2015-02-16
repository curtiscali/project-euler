import Data.Function
import Data.List
import qualified Data.Map as Map
import System.IO

chainLength :: (Integral a) => a -> a
chainLength n = chainLength' n 1 where
    chainLength' 1 acc = acc
    chainLength' x acc
        | even x = chainLength' (x `div` 2) $! (1 + acc)
        | otherwise = chainLength' (3 * x + 1) $! (1 + acc)

maxPair :: (Integral a) => [(a, a)] -> (a, a)
maxPair [] = error "Can't find max of empty list"
maxPair (x:xs) = maxPair' x xs where
    maxPair' a [] = a
    maxPair' a (y:ys)
        | snd a > snd y = maxPair' a ys
        | otherwise = maxPair' y ys

main = do
    let pairs = map (\n -> (n, chainLength n)) [2..1000000]
    print $ fst (maxPair pairs)