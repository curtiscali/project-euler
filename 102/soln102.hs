import System.IO
import Data.List.Split

data Vector = Vector {
    x :: Int,
    y :: Int
} deriving (Show)

vsub :: Vector -> Vector -> Vector
(Vector x1 y1) `vsub` (Vector x2 y2) = Vector (x1 - x2) (y1 - y2)

dotprod :: Vector -> Vector -> Int
(Vector x1 y1) `dotprod` (Vector x2 y2) = (x1 * x2) + (y1 * y2)


data Triangle = Triangle {
    a :: Vector,
    b :: Vector,
    c :: Vector
} deriving (Show)

contains :: Triangle -> Vector -> Bool
(Triangle a b c) `contains` point = (u >= 0) && (v >= 0) && (u + v < 1) where
    v1 = c `vsub` a
    v2 = b `vsub` a
    v3 = point `vsub` a
    dot11 = fromIntegral $ v1 `dotprod` v1
    dot12 = fromIntegral $ v1 `dotprod` v2
    dot13 = fromIntegral $ v1 `dotprod` v3
    dot22 = fromIntegral $ v2 `dotprod` v2
    dot23 = fromIntegral $ v2 `dotprod` v3
    invDenom = 1.0 / (dot11 * dot22 - dot12 * dot12)
    u = (dot22 * dot13 - dot12 * dot23) * invDenom
    v = (dot11 * dot23 - dot12 * dot13) * invDenom

fromLine :: String -> Triangle
fromLine line = Triangle {a=Vector x1 y1, b=Vector x2 y2, c=Vector x3 y3} where
    numbers = map (read :: String -> Int) (wordsBy (==',') line)
    x1 = head numbers
    y1 = numbers !! 1
    x2 = numbers !! 2 
    y2 = numbers !! 3 
    x3 = numbers !! 4 
    y3 = last numbers 

main = do
    handle <- openFile "triangles.txt" ReadMode
    contents <- hGetContents handle
    let triangles = map fromLine (lines contents)
        origin = Vector 0 0
    print $ sum [1 | t <- triangles, t `contains` origin]
