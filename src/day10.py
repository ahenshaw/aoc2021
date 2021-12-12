PATH = 'input/2021/day10.txt'
OPENER = '([{<'
CLOSER = ')]}>'
ERROR_SCORE = [3, 57, 1197, 25137]

total_error_score = 0
scores = []
for line in open(PATH):
    stack = []
    for ch in line.strip():
        if -1 != (index := OPENER.find(ch)):
            stack.append(index)
        elif stack.pop() != (index := CLOSER.find(ch)):
                total_error_score += ERROR_SCORE[index]
                break
    else:
        acc = 0; [acc := 5 * acc + x + 1 for x in stack[::-1]]
        scores.append(acc)

scores.sort()
print('part 1:', total_error_score)
print('part 2:', scores[len(scores) // 2])