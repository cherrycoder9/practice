# 불변성과 가변성
# 원본 리스트 변경 없이, 새로운 리스트에만 값이 추가되는 함수


def append_to_new_list(original_list: list, value) -> list:
    new_list = original_list.copy()
    new_list.append(value)
    return new_list


list = [1, 2, 3]
result = append_to_new_list(list, 4)
print(result)
print(list)
