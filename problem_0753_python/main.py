

def iterate_a_b_c(p):

    for a in range(1, p):
        for b in range(1, p):
            for c in range(1, p):
                yield (a, b, c)


def satisfy_equation(a, b, c, p):

    lhs = (a ** 3 + b ** 3) % p
    rhs = (c ** 3 ) % p

    is_equal = bool((lhs % p) == (rhs % p))
    return is_equal


def get_prime_list(limit):

    prime_list = []
    for n in range(2, limit + 1):
        is_prime = True
        for p in prime_list:
            if n % p == 0:
                is_prime = False
                break
        if is_prime:
            prime_list.append(n)
    
    return prime_list


def F(p):

    count = 0
    for a, b, c in iterate_a_b_c(p):
        if satisfy_equation(a, b, c, p):
            count += 1
    return count


print("F(5) = {}".format(F(5)))
print("F(7) = {}".format(F(7))) 
prime_list = get_prime_list(6000000)
print("Prime numbers: {} ({})".format(prime_list, len(prime_list)))

