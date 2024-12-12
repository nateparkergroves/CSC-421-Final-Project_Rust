import threading
import time

def compute_sum(start, end, results, index):
    results[index] = sum(range(start, end + 1))

if __name__ == "__main__":
    ranges = [
        (1, 50_000_000),
        (50_000_001, 100_000_000),
        (100_000_001, 150_000_000),
        (150_000_001, 200_000_000),
    ]

    results = [0] * len(ranges)
    threads = []

    start_time = time.time()  # Start the timer

    # Start threads for each range
    for i, (start, end) in enumerate(ranges):
        thread = threading.Thread(target=compute_sum, args=(start, end, results, i))
        threads.append(thread)
        thread.start()

    # Wait for all threads to complete
    for thread in threads:
        thread.join()

    total_sum = sum(results)
    end_time = time.time()  # End the timer

    print(f"Total sum: {total_sum}")
    print(f"Time taken: {end_time - start_time:.2f} seconds")
