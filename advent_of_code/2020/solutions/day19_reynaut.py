# code from @Reynouts
# Bedankt Reynouts! Door deze code snap ik wat er gebeurt

import re

def parse(data):
    todo = {}
    rules = {}
    messages = []
    for line in data:
        if ":" in line:
            n, r = line.split(": ")
            if "\"" in r:
                print(r[1])
                rules[int(n)] = r.strip()[1]
            else:
                todo[int(n)] = r.strip()
        elif line != "\n":
            messages.append(line.strip())
    return todo, rules, messages


def make_regex(todo, rules, p2=True):
    while 0 not in rules:
        done = []
        for t in todo:
            # t are numbers
            nms = set(map(int, re.findall("\d+", todo[t])))
            if all([True if n in rules else False for n in nms]):
                res = "("
                if t == 11 and p2:
                    for c in todo[t].split():
                        res += "(" + rules[int(c)] + "){x}"
                else:
                    for c in todo[t].split():
                        if c == "|":
                            res += c
                        else:
                            res += rules[int(c)]
                res += ")"
                if t == 8 and p2:
                    res += "+"
                done.append(t)
                rules[t] = res
        for d in done:
            todo.pop(d, None)
    return "^" + rules[0] + "$"


def count_valid(regex, messages):
    cnt = 0
    for i in range(1, 100):
        to_pop = []
        for x in messages:
            print(len(to_pop))
            if re.match(regex.replace("x", str(i)), x):
                to_pop.append(x)
                cnt += 1
        for x in to_pop:
            messages.remove(x)
    return cnt


def main():
    with open('../input.txt', 'r') as f:
        data = f.readlines()

    todo, rules, messages = parse(data)
    regex = make_regex(todo, rules, False)
    print("P1: {}".format(count_valid(regex, messages)))

    todo, rules, messages = parse(data)
    regex = make_regex(todo, rules, True)
    print("P2: {}".format(count_valid(regex, messages)))


if __name__ == "__main__":
    main()