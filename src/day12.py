from collections import defaultdict

PATH = 'input/2021/test12.txt'
PATH = 'input/2021/day12.txt'

def explore(edges, src, path, distinct, is_part_2):
    path.append(src)
    if src == 'end':
        distinct.add('-'.join(path))
        return
    for dest in edges[src]:
        lower = [x for x in path if not x.isupper()]
        any_doubles = len(lower) != len(set(lower))
        if dest != 'start':
            if dest.isupper() or (is_part_2 and not any_doubles and path.count(dest) < 2) or path.count(dest) < 1:
                explore(edges, dest, path.copy(), distinct, is_part_2)
edges = defaultdict(list)
for line in open(PATH):
    left, right = line.strip().split('-')
    edges[left].append(right)
    edges[right].append(left)

for i in [1, 2]:
    distinct = set()
    for dst in edges['start']:
        explore(edges, dst, [], distinct, is_part_2=(i==2))
    print(f'Part {i}:', len(distinct))