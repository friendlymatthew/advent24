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
            first, second = int(first), int(second)

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

            if not correct:
                # bubble sort
                n = len(pages)

                swapped = True

                while swapped:
                    swapped = False
                    for i in range(1, n):
                        left_page, right_page = pages[i - 1], pages[i]

                        if left_page not in page_rules or (right_page not in page_rules[left_page] and right_page in page_rules.keys()):
                            pages[i - 1], pages[i] = right_page, left_page
                            swapped = True


                out += int(pages[len(pages) // 2])

print(out)




