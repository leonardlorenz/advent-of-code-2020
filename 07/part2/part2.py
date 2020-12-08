bags = {}
def calculate(color_name):
    amount = 0
    if color_name == 'other': # edge case for bags not containing another
        return 0
    for subbag in bags[color_name]: # for all bags inside of a bag
        if subbag[1] != "other":
            amount += int(subbag[0]) + int(subbag[0]) * calculate(subbag[1])
    return amount

with open("./input.txt", "r") as f:
    tmp = map(lambda x: x.split(" contain "), f.read().replace(" bags", "").replace(" bag", "").split(".\n")[:-1])
    for (key, value) in tmp:
        can_contain = {}
        b = map(lambda x: x.strip(), value.strip().split(","))
        bags[key] = [(x[:1], x[2:].strip()) for x in b]

print(calculate("shiny gold"))
