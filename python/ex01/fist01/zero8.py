# 얕은 복사와 깊은 복사
# 중첩된 리스트가 포함된 원본 리스트를 깊은 복사한 뒤
# 복사본의 내부 리스트의 값을 변경하는 함수 작성
# 함수는 원본 리스트와 수정된 복사본 리스트를 반환해야 함

import copy


def deep_copy_and_modify(original_list: list) -> tuple:
    deep_copy_list = copy.deepcopy(original_list)
    deep_copy_list[1][0] = 5
    return (original_list, deep_copy_list)


list = [1, [2, 3], 4]
result = deep_copy_and_modify(list)
print(result)
