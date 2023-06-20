#!/usr/bin/python

import sys
import matplotlib.pyplot as plt

col_x = []
col_y = []

for line in sys.stdin:
    data = line.strip().split('\t')
    col_x.append(float(data[0]))
    col_y.append(float(data[1]))

fig, ax = plt.subplots()
plt.scatter(col_x, col_y, linewidth=0.5)
plt.show()
