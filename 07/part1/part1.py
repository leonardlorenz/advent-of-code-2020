def calculate(color_name, bags):
    if color_name == 'other': # edge case for bags not containing another
        return False
    for i in bags[color_name]: # for all bags inside of a bag
        if i[1] == 'shiny gold':
            return True
        else:
            if calculate(i[1], bags):
                return True
    return False

with open("./input.txt", "r") as f:
    bags = {}
    tmp = map(lambda x: x.split(" contain "), f.read().replace(" bags", "").replace(" bag", "").split(".\n")[:-1])
    for (key, value) in tmp:
        can_contain = {}
        b = map(lambda x: x.strip(), value.strip().split(","))
        bags[key] = [(x[:1], x[2:].strip()) for x in b]
    amount = 0
    #for key, value in bags.items():
    #    print(key, value)
    for color in bags.keys():
        if calculate(color, bags):
            amount += 1
    print(amount)
