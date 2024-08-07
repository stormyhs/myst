import time

start = time.perf_counter()

i = 0
max = 100_000

while i < max:
    i += 1

end = time.perf_counter()
print(f"Time taken: {end - start} seconds")
