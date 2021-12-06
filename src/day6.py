fishes = [int(x) for x in open('input/2021/day6.txt').read().split(',')]

# for d in range(80):
#     addon = []
#     for i, fish in enumerate(fishes):
#         if fish == 0:
#             fishes[i] = 6
#             addon.append(8)
#         else:
#             fishes[i] = fish - 1
#     fishes.extend(addon)  


def next_day(ages):
    first = ages.pop(0)
    ages[6] += first
    ages.append(first)
    return ages

ages = [0]*9

for d in fishes:
    ages[d] += 1

for i in range(256):
    ages = next_day(ages)                                                                                      
print(sum(ages))