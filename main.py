import time

# // 10,000,000 Values
data: list[str] = [1 for _ in range(10 ** 7)]

# // While Loop
def while_loop():
    start_time: int = time.time()
    i: int = 0
    while i < len(data):
        i += 1
    print("\nWhile Loop: "+str(time.time() - start_time))


# // Counted loop
def counted_for_loop():
    start_time: int = time.time()
    for i in range(len(data)):
        i += 0
    print("\nCounted For Loop: "+str(time.time() - start_time))


# // Variable Loop
def for_loop():
    start_time: int = time.time()
    for i in data:
        i += 0
    print("\nFor Loop: "+str(time.time() - start_time))


# // Run the functions
if __name__ =="__main__":
    while_loop()
    counted_for_loop()
    for_loop()
    