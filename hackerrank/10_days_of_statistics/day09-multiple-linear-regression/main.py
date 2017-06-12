#!/usr/bin/env python

from sklearn import linear_model
import numpy as np

def main():
    m, n = [int(x) for x in input().strip().split(' ')]
    xy = np.array([list(map(float, input().strip().split(' ')))\
                   for _ in range(n)])\
           .reshape((n, m + 1))

    x = xy.copy()[:, :-1]
    y = xy.copy()[:, -1]

    regr = linear_model.LinearRegression()
    regr.fit(x, y)

    q = int(input().strip())
    for _ in range(q):
        f = np.array(list(map(float, input().strip().split(' '))))\
              .reshape(1, -1)
        print("%.2f" % regr.predict(f)[0])

if __name__ == '__main__':
    main()
