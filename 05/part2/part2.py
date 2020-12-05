def consume(string, value_range):
    if string == "":
        return value_range[0]
    if string[0] == 'F' or string[0] == 'L': # lower half
        return consume(string[1:], value_range[:int(len(value_range) / 2)])
    else: # upper half
        return consume(string[1:], value_range[int(len(value_range) / 2):])

with open("./input.txt", "r") as f:
    seats = [(i[:7].strip(), i[7:].strip()) for i in f.readlines()]

ids = []
for s in seats:
    row_value_range = [i for i in range(0, 128)]
    col_value_range = [i for i in range(0, 8)]
    row = consume(s[0], row_value_range)
    col = consume(s[1], col_value_range)
    ids.append(row * 8 + col)

ids_cropped = sorted(ids)[1:-1]
for i in range(ids_cropped[0], ids_cropped[-1]):
    if i not in ids_cropped:
        print(i)
