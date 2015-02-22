#!/usr/bin/env runhaskell

digitSum :: (Integral a) => a -> a
digitSum n = digitSum' n 0 where
    digitSum' 0 acc = acc
    digitSum' n acc = digitSum' (n `div` 10) ((n `mod` 10) + acc)

main = print $ digitSum (2^1000)