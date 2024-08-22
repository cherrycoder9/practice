# 얕은 복사와 깊은 복사
# 얕은 복사: 객체의 참조만 복사되어 원본과 복사본이 같은 가변 객체를 참조함
# 깊은 복사: 객체의 내용을 복사해 독립적인 객체를 생성함

import copy

# 얕은 복사
original_list = [1, 2, [3, 4]]
shallow_copy = copy.copy(original_list)
shallow_copy[2][0] = 99

print(original_list)  # [1, 2, [99, 4]]

# 깊은 복사
original_list = [1, 2, [3, 4]]
deep_copy = copy.deepcopy(original_list)
deep_copy[2][0] = 99

print(original_list)  # [1, 2, [3, 4]]
