a = 7
b = 8
prime = 13


def find_xy(a, b, prime):
    for x in range(prime):
        for y in range(prime):
            if (y ** 2) % prime == (x ** 3 + a * x + b) % prime:
                print(f"x = {x}, y = {y}")
