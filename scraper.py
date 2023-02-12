# from selenium import webdriver
# from selenium.webdriver.chrome import options
from bs4 import BeautifulSoup
from bs4.element import PageElement
from dataclasses import dataclass
import requests

# op = options.Options()
# op.headless = True
# op.add_argument("--log-level=3")
# browser = webdriver.Chrome("D:/setup/chromedriver_win32/chromedriver.exe", chrome_options=op)


@dataclass
class Definition:
    part_of_speech: str
    category: str
    explanation: str


@dataclass
class Data:
    word: str
    definitions: list[Definition]


def __parse(element: PageElement) -> str:
    return element.get_text().strip()


def __strip(target: list) -> list:
    out = []
    for v in target:
        if v != '\n':
            out.append(v)
    return out


def __children(element: PageElement) -> list[PageElement]:
    return __strip(list(element.children))


def go(word: str) -> BeautifulSoup:
    res = requests.get(f"https://www.wordnik.com/words/{word}")
    return BeautifulSoup(res.text, "html.parser")


def get(word: str) -> Data:
    soup = go(word)
    response = soup.find("div", class_="guts").find_all("ul")
    definitions: list[Definition] = []
    for row in response:
        for cell in __children(row):
            category = cell.find("i").get_text()
            splitter = 1 if category == '' else 2
            explanation = " ".join(" ".join(list(cell.stripped_strings)[splitter:]).split("  "))
            d = Definition(
                cell.find("abbr").get_text(),
                category,
                explanation
            )
            if d.explanation != "":
                definitions.append(d)
    return Data(
        word,
        definitions
    )
