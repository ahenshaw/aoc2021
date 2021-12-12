PATH = 'input/2021/test11.txt'
PATH = 'input/2021/day11.txt'

data = open(PATH).read().replace('\n', '')
data = [int(x) for x in data]
import numpy as np
a = np.array(data, dtype=int).reshape((10, 10))
offset = [(0, 1), (0, -1), (1, 0), (-1,0), (-1, -1), (-1, 1), (1, -1), (1, 1), (0, 0)]
count = 0
for step in range(1000):
    a += 1
    flashed = set()
    while True:
        flasher = set([tuple(x) for x in np.column_stack(np.where(a>9))]) - flashed
        if not len(flasher):
            break
        # print('---')
        flashed = flashed.union(flasher)
        for row, col in flasher:
            # print(row, col)
            for dr, dc in offset:
                r = row + dr; c = col + dc
                if r >= 0 and r < 10 and c>=0 and c<10:
                    a[r, c] += 1
    # print(a)
    a[np.where(a>9)] = 0
    if np.all(a==0):
        print('All flash', step+1)
        break
    count += len(np.column_stack(np.where(a==0)))
    # print(step, count)
    # print(a)
            
print(count)