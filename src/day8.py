from collections import Counter
segments = {3:7, 4:4, 2:1, 7:8}
count = Counter()
map = {}
for line in open('input/2021/test8.txt'):
# for line in open('input/2021/day8.txt'):
    inp, out = line.split('|')
    for o in out.split():
        try:
            m = segments[len(o)]
            count[m] += 1
        except KeyError:
            pass
print(sum(count.values()))

total = 0
# for line in open('input/2021/test8.txt'):
for line in open('input/2021/day8.txt'):
    dd = {}
    inp, out = line.split('|')
    print(inp, flush=True)
    undecoded = set(inp.split())
    while undecoded:
        drop = set()
        for sig in undecoded:
            ssig = set(sig)
            try:
                if len(sig) in segments:
                    dd[segments[len(sig)]] = ssig
                    drop.add(sig)
                    dd['a'] = (dd[8]-dd[4]).intersection(dd[7])
                elif len(sig) == 6:
                    x = dd[8] - ssig - dd[1]
                    if not x:
                        dd[6] = ssig
                        drop.add(sig)
                        dd['c'] = dd[8] - dd[6]
                        dd['f'] = dd[6].intersection(dd[1] - dd['c'])
                    else:
                        x = dd[8] - ssig - dd[4]
                        if x:
                            dd[9] = ssig
                            dd['e'] = dd[8] - dd[9]
                        else:
                            dd[0] = ssig
                            dd['d'] = dd[8] - dd[0]
                        drop.add(sig)
                else: #2, 3, 5
                    if ssig.intersection(dd[1]) == dd[1]:
                        dd[3] = ssig
                        dd['b'] = dd[8] - dd[3] - dd['e']
                        drop.add(sig)
                    elif ssig.intersection(dd['b']):
                        dd[5] = ssig
                        drop.add(sig)
                    else:
                        dd[2] = ssig
                        drop.add(sig)
                    
                     
            except KeyError:
                continue
        undecoded -= drop
    num = ''
    de = {}
    for k, v in dd.items():
        de[tuple(sorted(v))] = str(k)
    for o in out.split():
        num += de[tuple(list(sorted(o)))]
    total += int(num)
print(total)
    # print(dd)



# data = [int(x) for x in open('input/2021/day8.txt').read().split(',')]
#   0:      1:      2:      3:      4:
#  aaaa    ....    aaaa    aaaa    ....
# b    c  .    c  .    c  .    c  b    c
# b    c  .    c  .    c  .    c  b    c
#  ....    ....    dddd    dddd    dddd
# e    f  .    f  e    .  .    f  .    f
# e    f  .    f  e    .  .    f  .    f
#  gggg    ....    gggg    gggg    ....

#   5:      6:      7:      8:      9:
#  aaaa    aaaa    aaaa    aaaa    aaaa
# b    .  b    .  .    c  b    c  b    c
# b    .  b    .  .    c  b    c  b    c
#  dddd    dddd    ....    dddd    dddd
# .    f  e    f  .    f  e    f  .    f
# .    f  e    f  .    f  e    f  .    f
#  gggg    gggg    ....    gggg    gggg