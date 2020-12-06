def unique(it):
        s = set()
        for el in it:
            if el not in s:
                s.add(el)
                yield el

with open("./input.txt", "r") as f:
    groups = [i.split("\n") for i in f.read().split("\n\n")]
    count_sum = 0
    for group in groups:
        answers = ""
        for person in group:
            answers += person
        count_sum += len([el for el in unique(list(answers))])
    print(count_sum)




