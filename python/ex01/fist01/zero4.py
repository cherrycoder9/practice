# 타입 힌팅
# 파이썬 3.5에 도입된 타입 힌팅은 코드 가독성을 높이고
# IDE나 린터가 타입 검사를 할 수 있도록 함
def add_numbers(a: int, b: int) -> int:
    return a + b


result = add_numbers(10, 20)
print(result)
