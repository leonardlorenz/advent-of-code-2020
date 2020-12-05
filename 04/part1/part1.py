with open("./input.txt", "r") as f:
    lines = f.read()
    passports = lines.split("\n\n")
    valid_passports = 0
    for p in map(lambda x: x.replace(" ", "\n").split("\n"), passports):
        if len(p) == 8:
            valid_passports += 1
            continue
        p_identifiers = sorted([item.split(":")[0] for item in p if len(p) == 7])
        if p_identifiers == ['byr', 'ecl', 'eyr', 'hcl', 'hgt', 'iyr', 'pid']:
            valid_passports += 1
    print(valid_passports)
