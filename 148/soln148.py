def pascal(n):
    line = [1]
    for i in range(n):
        line.append(line[i] * (n-i) // (i+1))
    
    return line

def main():
    print(numEntries)

if __name__ == '__main__':
    main()