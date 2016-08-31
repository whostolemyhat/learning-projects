"""Finds pairs of prime numbers which add up to a target number"""
import sys

# ooh type annotations
def create_sieve(limit: int) -> [bool]:
    """Creates a Sieve of Eratosthenes"""

    primes = [True] * (limit + 1)
    primes[0] = primes[1] = False

    for(i, is_prime) in enumerate(primes):
        if is_prime:
            # if you want a generator, yield here
            # we need to work from both ends, so use a list
            # yield i
            for num in range(i * i, limit, i):
                primes[num] = False

    return primes

def main():
    if len(sys.argv) < 2:
        input_string = input("Enter a target number: ")
    else:
        input_string = sys.argv[1]

    target = int(input_string)
    sieve = create_sieve(target)

    results = []
    for i in range(1, int(len(sieve) / 2) + 1):
        if sieve[i] and sieve[target - i]:
            results.append((i, target - i))

    print(results)

if __name__ == '__main__':
    main()