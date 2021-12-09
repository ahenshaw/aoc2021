import numpy as np

d = []
for line in open('input/2021/test9.txt'):
    d.append([int(x) for x in line.strip()])
# data = [int(x) for x in open('input/2021/day9.txt').read().split(',')]

d = np.array(d, dtype=int)
d = np.pad(d, 1, 'constant', constant_values=(11,))
total = 0
rows, cols = d.shape
for r  in range(1, rows-1):
    for c in range(1, cols-1):
        if (d[r][c] < d[r-1][c]) and (d[r][c] < d[r+1][c]) and (d[r][c] < d[r][c-1]) and (d[r][c] < d[r][c+1]):
            total += d[r][c] + 1
print(total)


def find_conn(d, r, c, visited):
    offsets = [(0,-1), (0, 1), (-1, 0), (1, 0)]
    visited.add((r, c))
    test = d[r][c]
    size = 1
    for dr, dc in offsets:
        row = r + dr
        col = c + dc
        if (row, col) not in visited:
            if d[row][col] != 9 and d[row][col]> test:
                size += find_conn(d, row, col, visited)
    return size

d = []
for line in open('input/2021/day9.txt'):
    d.append([int(x) for x in line.strip()])
# data = [int(x) for x in open('input/2021/day9.txt').read().split(',')]

d = np.array(d, dtype=int)
d = np.pad(d, 1, 'constant', constant_values=(9,))
print(d)
total = 0
rows, cols = d.shape
sizes = []
for r  in range(1, rows-1):
    for c in range(1, cols-1):
        if (d[r][c] < d[r-1][c]) and (d[r][c] < d[r+1][c]) and (d[r][c] < d[r][c-1]) and (d[r][c] < d[r][c+1]):
            sizes.append(find_conn(d, r, c, set()))
a = sorted(sizes, reverse=True)[:3]
print(a[0]*a[1]*a[2])

