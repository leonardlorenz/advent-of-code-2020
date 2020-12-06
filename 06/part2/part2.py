def unique(it):
        s = set()
        for el in it:
            if el not in s:
                s.add(el)
                yield el

with open("./input.txt", "r") as f:
    groups = [i.split("\n") for i in f.read().split("\n\n")]
    sum_count = 0
    for group in groups:
        answers = {}
        for person in group:
            for answer in list(person):
                if not answer in answers:
                    answers[answer] = 0
                else:
                    answers[answer] += 1
        for answer in answers:
            if answers[answer] == len(group) - 1:
                sum_count += 1
    print(sum_count)
