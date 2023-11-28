def main():
    list_of_elves = []
    cur_cal = 0
    with open("input", "r") as f:
        l = f.readline();
        while l != "":
            if l == "\n":
                list_of_elves.append(cur_cal)
                cur_cal = 0
            else:
                cur_cal += int(l)
            l = f.readline();
    list_of_elves.sort()
    print(list_of_elves[-3:])


if __name__ == "__main__":
    main()