import numpy as np
from numpy.lib.function_base import average
data = [int(x) for x in open('input/2021/test7.txt').read().split(',')]
data = [int(x) for x in open('input/2021/day7.txt').read().split(',')]

data = np.array(data)
def calc(data, pos):
    return np.sum(np.abs(data - pos))

def calc2(data, pos):
    dx_all= np.abs(data - pos)
    cost = 0
    for i, dx in enumerate(dx_all):
        cost += ((dx + 1)* dx)//2
        # for x in range(dx+1):
        #     cost += x
            # if pos == 5:
            #     print(i, data[i], dx, cost)
    return cost


import time
start = time.perf_counter_ns()
cost = []
for i in range(max(data)):
    cost.append(calc2(data, i)) 
x = np.argmin(cost)
print(time.perf_counter_ns() - start)
print(x, cost[x])
