sumOfSquares :: (Integral a) => [a] -> a
sumOfSquares xs = sum [x * x | x <- xs]

squareOfSum :: (Integral a) => [a] -> a
squareOfSum xs = listSum * listSum where listSum = sum xs

main = do
    let list = [1..100]
    let difference = squareOfSum list - sumOfSquares list
    print difference