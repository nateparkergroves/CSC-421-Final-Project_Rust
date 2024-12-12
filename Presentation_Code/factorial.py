import time

def factorial(n):
    if n == 0:
        return 1
    else:
        return n * factorial(n - 1)

def main():
    start_time = time.time()
    
    results = []
    for i in range(1, 21):
        results.append(factorial(i))
    
    end_time = time.time()
    
    print("Results:", results)
    print("Time taken:", end_time - start_time, "seconds")

if __name__ == "__main__":
    main()
