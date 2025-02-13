from json import loads, dump


def main():
    data = {}
    for i in range(0, 370100, 100):
        with open(f'raw_data/{i+100}.json', 'r') as f:
            data.update(loads(f.read()))

    filtered = {}
    for cell in data:
        if data[cell]:
            filtered.update({cell: data[cell]})

    data_set = {}
    current = "a"
    for cell in filtered:
        if cell[0] != current:
            with open(f'data/{current}.json', 'w') as f:
                dump(data_set, f)
            current = cell[0]
            data_set.clear()
        data_set.update({cell: filtered[cell]})

    with open(f'data/{current}.json', 'w') as f:
        dump(data_set, f)
    data_set.clear()


if __name__ == "__main__":
    main()