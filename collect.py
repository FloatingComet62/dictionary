import scraper
from json import dump
from multiprocessing import Pool


def load_words(part: int = None) -> list[str]:
    with open('words.txt') as f:
        out = list(set(f.read().split()))
    out.sort()
    if part:
        return out[:part]
    return out


def run(data: tuple[int, int]):
    skip, limit = data
    words = load_words()
    num_words = len(words)
    data_set = {}

    for i, word in enumerate(words[skip:]):
        if i == limit:
            break
        if (i + skip + 1) % 100 == 0:
            with open(f'raw_data/{i + skip + 1}.json', 'w') as f:
                dump(data_set, f)
            data_set = {}
            print(f"---{i + skip + 1}---")

        try:
            data = scraper.get(word)
        except:
            data = scraper.Data(word, [])
        data_set.update({
            word: [
                {
                    "part_of_speech": d.part_of_speech,
                    "category": d.category,
                    "explanation": d.explanation
                } for d in data.definitions
            ]
        })
        print(f"{i + skip} : {(i + skip) * 100 / num_words}")


def main():
    check_point = 65700
    with Pool(processes=150) as pool:
        results = pool.imap_unordered(run, [(check_point + i, 100) for i in range(0, (370105 - check_point), 100)])

        for _ in results:
            pass


if __name__ == "__main__":
    # run((100, 1000)) -> collect from 100 to 1100
    main()  # -> ALL
