def is_palindrome(s:string) -> bool:
    return all(s[i] == s[-i-1] for i in range(len(s)//2))

def solve_largest_palindrome_product(digits: int) -> int:
    hi, lo = 10**digits - 1, 10**(digits-1)
    max_p = 0
    for a in range(hi, lo - 1, -1):
        for b in range(hi, lo - 1, -1):
            prod = a * b
            if prod <= max_p:
                break
            if is_palindrome(str(prod)):
                print(a,b)
                max_p = prod
    return max_p


if __name__ == "__main__":
    # result2 = solve_largest_palindrome_product(2)
    # assert result2 == 9009
    # print(f"result is {result2}")
    result = solve_largest_palindrome_product(3)
    print(f"result is {result}")
