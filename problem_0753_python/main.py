



def iterate_a_b_c(p):

    for a in range(1, p):
        for b in range(1, p):
            for c in range(1, p):
                yield (a, b, c)


def satisfy_equation(a, b, c, p):

    lhs = (a ** 3 + b ** 3) % p
    rhs = (c ** 3 ) % p

    is_equal = bool((lhs % p) == (rhs % p))
    #if is_equal:
    #    print(f"{a}^3 + {b} ^ 3 = {lhs} == {rhs} = {c}^3")
    #else:
    #    print(f"{a}^3 + {b} ^ 3 = {lhs} <> {rhs} = {c}^3")
    return is_equal


def F(p):

    count = 0
    for a, b, c in iterate_a_b_c(p):
        if satisfy_equation(a, b, c, p):
            count += 1
    return count

print("F(5) = {}".format(F(5)))
print("F(7) = {}".format(F(7))) 
print("F(1000) = {}".format(F(1000))) 

