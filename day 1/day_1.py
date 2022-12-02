

if __name__ == '__main__':
    with open('input.txt') as f:
        data = f.read().splitlines()
    max_cal = 0
    cal = 0
    totals = []
    for line in data:
        if not line:
            # New line
            if cal > max_cal:
                max_cal = cal
            totals.append(cal)
            cal = 0
            continue
        # Parse data
        cal += int(line)
    if cal > max_cal:
        max_cal = cal
    # print(max_cal, cal)
    print(sorted(totals)[-3:])
    print(sum(sorted(totals)[-3:]))