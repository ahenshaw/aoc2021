import numpy as np

def sign(a): return bool(a > 0) - bool(a < 0)

def draw_line(pt1, pt2):
    (x1, y1), (x2, y2) = sorted([pt1, pt2])
    dx = sign(x2 - x1)
    dy = sign(y2 - y1)
    x, y = x1, y1
    while True:
        grid[y][x] += 1
        if x == x2 and y == y2: break
        x += dx; y += dy

m = 0
lines = []
for line in  open('input/2021/day5.txt'):
    left, right = line.strip().split('->')
    pt1 = [int(a) for a in left.split(',')]
    pt2 = [int(a) for a in right.split(',')]
    m = max(m, max(pt1), max(pt2))
    lines.append((pt1, pt2))
m += 1

grid = np.zeros((m, m), dtype=int)
for pt1, pt2 in lines:
    if pt1[0] == pt2[0] or pt1[1] == pt2[1]:
        draw_line(pt1, pt2)
print('part1', np.sum(grid >=2))

grid = np.zeros((m, m), dtype=int)
for pt1, pt2 in lines:
    draw_line(pt1, pt2)
print('part2', np.sum(grid >=2))