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
            splitted = line.split("|")

            if len(splitted) < 2:
                print("improper")
                break

            first = int(splitted[0])
            second = int(splitted[1])

            if first not in page_rules:
                pages = set()
                pages.add(second)
                page_rules[first] = pages 
            else:
                page_rules[first].add(second)
        else:
            # now we dealing with the actual pages
            pages = line.split(",")

            correct = True 

            for curr_i, curr_page in enumerate(pages):
                curr_page = int(curr_page)

                for next_page in pages[curr_i + 1:]:
                    next_page = int(next_page)

                    if next_page in page_rules:
                        if curr_page in page_rules[next_page]:
                            correct = False
                            break

            if correct:
                out += int(pages[len(pages) // 2]) 

print(out)




