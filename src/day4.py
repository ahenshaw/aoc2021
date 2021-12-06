import numpy as np
import sys

data = open('input/2021/day4.txt').read().split('\n\n')
draw = [int(x) for x in  data[0].split(',')]

boards = []
for group in data[1:]:
    board = np.array([int(x) for x in group.split()]).reshape((5,5))
    boards.append(board)

winners = set()
for card in draw:
    if len(winners) == len(boards):
        break
    print(card)
    for i, board in enumerate(boards):
        if i not in winners:
            board[np.where(board == card)] = 100
            if np.any(np.sum(board, 0) == 500) or np.any(np.sum(board, 1)==500):
                board[np.where(board==100)] = 0
                print(np.sum(board)*card)
                winners.add(i)


# for board in boards:
#     print(board)
#     print()