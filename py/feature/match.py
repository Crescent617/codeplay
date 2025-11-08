#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
结构化模式匹配 (match-case) 示例合集
Python 3.10+
"""

def line(title: str):
    print("\n" + "=" * 10, title, "=" * 10)

# 1️⃣ 基础匹配
def test_basic():
    line("基础匹配")

    def describe(x):
        match x:
            case 0:
                return "zero"
            case 1 | 2:
                return "one or two"
            case _:
                return f"something else: {x}"

    for v in [0, 1, 3]:
        print(f"{v!r} → {describe(v)}")


# 2️⃣ 元组匹配
def test_tuple():
    line("元组匹配")

    points = [(0, 0), (2, 0), (0, 5), (3, 4)]
    for p in points:
        match p:
            case (0, 0):
                print("origin")
            case (x, 0):
                print(f"x-axis at {x}")
            case (0, y):
                print(f"y-axis at {y}")
            case (x, y):
                print(f"point ({x}, {y})")


# 3️⃣ 列表匹配
def test_list():
    line("列表匹配")

    data_list = [[1], [1, 2], [1, 2, 3, 4]]
    for data in data_list:
        match data:
            case [x]:
                print(f"单元素列表: {x}")
            case [x, y]:
                print(f"两个元素: {x}, {y}")
            case [x, *rest]:
                print(f"首元素 {x}, 剩余 {rest}")


# 4️⃣ 类匹配
def test_class():
    line("类匹配")

    class Point:
        __match_args__ = ("x", "y")

        def __init__(self, x, y):
            self.x = x
            self.y = y

    def locate(p):
        match p:
            case Point(0, 0):
                return "origin"
            case Point(x, 0):
                return f"x-axis at {x}"
            case Point(x, y):
                return f"point ({x}, {y})"

    for p in [Point(0, 0), Point(2, 0), Point(1, 1)]:
        print(locate(p))


# 5️⃣ 字典匹配
def test_dict():
    line("字典匹配")

    configs = [
        {"type": "http", "port": 8080},
        {"type": "unix", "path": "/tmp/socket"},
        {"type": "ftp"},
    ]

    for c in configs:
        match c:
            case {"type": "http", "port": p}:
                print(f"HTTP on port {p}")
            case {"type": "unix", "path": path}:
                print(f"Unix socket at {path}")
            case _:
                print("Unknown config")


# 6️⃣ 守卫条件
def test_guard():
    line("守卫条件")

    for n in [-3, 0, 7]:
        match n:
            case v if v < 0:
                print(f"{v}: negative")
            case v if v == 0:
                print(f"{v}: zero")
            case _:
                print(f"{n}: positive")


# 7️⃣ 类型匹配
def test_type():
    line("类型匹配")

    values = [42, "123", "abc", 3.14]

    for value in values:
        match value:
            case int():
                print(f"{value!r} → 整数")
            case str() as s if s.isdigit():
                print(f"{value!r} → 数字字符串")
            case str():
                print(f"{value!r} → 普通字符串")
            case _:
                print(f"{value!r} → 其他类型")


# 8️⃣ 综合例子：事件路由
def test_event():
    line("综合例子：事件路由")

    events = [
        {"type": "click", "pos": (10, 20)},
        {"type": "keypress", "key": "q"},
        {"type": "keypress", "key": "a"},
    ]

    def handle(event):
        match event:
            case {"type": "click", "pos": (x, y)}:
                print(f"clicked at {x}, {y}")
            case {"type": "keypress", "key": "q"}:
                print("quit")
            case {"type": "keypress", "key": k}:
                print(f"pressed key {k}")
            case _:
                print("unknown event")

    for e in events:
        handle(e)


# 🏁 主函数
if __name__ == "__main__":
    test_basic()
    test_tuple()
    test_list()
    test_class()
    test_dict()
    test_guard()
    test_type()
    test_event()
