PATH = 'input/2021/day13.txt'
grid = set()
for line in open(PATH):
    if line.startswith('fold'):
        fold = int(line.split('=')[1])
        folded = set()
        for (x, y) in grid:
            y = (2 * fold - y) if 'y' in line and y > fold else y
            x = (2 * fold - x) if 'x' in line and x > fold else x
            folded.add((x, y))
        grid = folded
        print('Active:', len(grid))
    elif line := line.strip():
        grid.add(tuple((int(x) for x in line.split(','))))

for y in range(max([v[1] for v in grid])+1):
    for x in range(max([v[0] for v in grid])+1):
        print('#' if (x, y) in grid else ' ', end='')
    print()