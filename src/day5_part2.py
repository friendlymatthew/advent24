with open("./tests/day5.txt") as f:
    input = f.read()

lines = input.splitlines()
out = 0

# parse the sections
page_rules = {}

section_one = True 

for line in lines:
    if len(line) == 0:
        print(page_rules)
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

            if not correct:
                # bubble sort
                n = len(pages)

                swapped = True

                while swapped:
                    swapped = False
                    for i in range(1, n):
                        left_page = int(pages[i - 1])
                        right_page = int(pages[i])
                        
                        # swap
                        # left | right
                        # if left is not in right
                        # 
                        # and right is not in the page_rules 
                        

                        if left_page not in page_rules or (right_page not in page_rules[left_page] and right_page in page_rules.keys()):
                            tmp_page = pages[i]
                            pages[i] = left_page
                            pages[i - 1] = right_page
                            swapped = True

                print(pages)

                out += int(pages[len(pages) // 2])

print(out)




