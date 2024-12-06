with open("./tests/day5.txt") as f:
    input = f.read()

lines = input.splitlines()
out = 0

# parse the sections
page_rules = {}

section_one = True 

for line in lines:
    if len(line) == 0:
        section_one = False
    else:
        if section_one:
            first, second = line.split("|")

            if first not in page_rules:
                page_rules[first] = set([second]) 
            else:
                page_rules[first].add(second)
        else:
            # now we dealing with the actual pages
            pages = list(int(x) for x in line.split(","))

            correct = True 

            for curr_i, curr_page in enumerate(pages):
                for next_page in pages[curr_i + 1:]:
                    if next_page in page_rules:
                        if curr_page in page_rules[next_page]:
                            correct = False
                            break

            if correct:
                out += int(pages[len(pages) // 2]) 

print(out)




