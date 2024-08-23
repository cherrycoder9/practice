# 변수 타입 변환
# 두 개의 문자열로 표현된 정수 값을 입력받아, 두 정수를 더한 값을 반환하는 함수 작성
def add_string_numbers(num1: str, num2: str) -> int:
    return int(num1) + int(num2)


result = add_string_numbers("1", "2")
print(result)
