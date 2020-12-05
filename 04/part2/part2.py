import re
with open("./input.txt", "r") as f:
    valid_hcl = re.compile(r"^#([0-9]|[a-f]){6}$")
    valid_pid = re.compile(r"^ *[0-9]{9}$")
    valid_hgt = re.compile(r"^[0-9]*(cm|in)$")
    lines = f.read()
    passports = lines.split("\n\n")
    valid_passports = 0
    for p in map(lambda x: x.replace(" ", "\n").split("\n"), passports):
        if len(p) < 7:
            continue
        p_identifiers = [item.split(":")[0] for item in p if len(p) == 7]
        if "cid" in p_identifiers:
            continue
        key_val = {}
        for key, val in [i.split(":") for i in p if i != ""]:
            key_val[key] = val
        byr, iyr, eyr = int(key_val["byr"]), int(key_val["iyr"]), int(key_val["eyr"])
        if byr < 1920 or byr > 2002 or \
           iyr < 2010 or iyr > 2020 or \
           eyr < 2020 or eyr > 2030:
            continue
        if not valid_hgt.match(key_val["hgt"]):
            continue
        hgt = int(key_val["hgt"][:-2])
        if key_val["hgt"].endswith("cm"):
            if hgt < 150 or hgt > 193:
                continue
        elif key_val["hgt"].endswith("in"):
            if hgt < 59 or hgt > 76:
                continue
        else:
            continue
        if not valid_hcl.match(key_val["hcl"]):
            continue
        if key_val["ecl"] not in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]:
            continue
        if not valid_pid.match(key_val["pid"]):
            continue
        valid_passports += 1
    print(valid_passports)
