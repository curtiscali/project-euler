import Data.Function
import Data.List
import Data.List.Split
import System.IO
import System.Environment

eval :: (Floating a) => (a, a) -> a
eval (a, b) = b * log a

createExponentPair :: String -> (Float, Float)
createExponentPair numbers =
    let splitNumbers = wordsBy (==',') numbers in
    (read (head splitNumbers) :: Float, read (last splitNumbers) :: Float)

main = do
    handle <- openFile "base_exp.txt" ReadMode  
    contents <- hGetContents handle  
    let exponents = map createExponentPair (lines contents)
        largest = maximumBy (compare `on` eval) exponents
        index = elemIndex largest exponents
    case index of
        Just n -> print $ n + 1
        Nothing -> putStrLn "Nothing found dawg"