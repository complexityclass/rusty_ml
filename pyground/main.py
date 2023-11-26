import rs_ml


def main():
    a = 5
    b = 6
    res = rs_ml.sum_as_string_sevens(a, b)

    print(f"result is: {res}")

    a = [1, 2, 3, 4, 5]
    b = [2, 3, 4, 5, 6]

    c = [x for x in zip(a, b)]
    print(c)


if __name__ == "__main__":
    main()
