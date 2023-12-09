import rs_ml
import numpy as np


def main():
    X = np.array([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]])
    y = np.array([1.0, 2.0, 3.0])
    rs_ml.knn(X, y)


if __name__ == "__main__":
    main()
