# 리스트 병합과 반복
# 새로운 리스트 생성시 유용한 방법

list1 = [1, 2, 3]
list2 = [4, 5, 6]

# 리스트 병합
merged_list = list1 + list2
print(merged_list)

# 리스트 반복
repeated_list = list1 * 3
print(repeated_list)

# 리스트 언패킹
# 리스트의 요소를 여러 변수에 할당
my_list = [1, 2, 3]
# 언패킹
a, b, c = my_list
print(a, b, c)
# 언패킹에서의 확장
# 연산자 *가 붙은 변수는 리스트나 튜플에서 남은 모든 요소를 받아들임
a, *middle, c = [1, 2, 3, 4, 5]
print(a, middle, c)

# 리스트와 이터레이터
# zip과 리스트를 사용해 두 리스트 결합
list1 = [1, 2, 3]
list2 = ['a', 'b', 'c']
# zip() 함수는 파이썬의 내장 함수
# 여러 개의 이터러블 객체의 요소들을 모아서 튜플로 묶어주는 역할
# 각 이터러블 객체의 동일한 위치에 있는 요소들을 하나의 튜플로 묶어주고 반환
zipped = zip(list1, list2)
print(zipped)  # <zip object at 0x000002508CDCF980>
zipped_list = list(zipped)
print(zipped_list)  # [(1, 'a'), (2, 'b'), (3, 'c')]

# zip()으로 이터러블 객체 분리
pairs = [(1, 'a'), (2, 'b'), (3, 'c')]
list1, list2 = zip(*pairs)
print(list1)  # (1, 2, 3)
print(list2)  # ('a', 'b', 'c')

# 리스트의 메모리 사용량 확인
import sys

large_list = [0] * 10000
print(sys.getsizeof(large_list))

# 리스트 복사에 대한 고려
import copy

list1 = [1, 2, 3]
shallow_copy = list1  # 얕은 복사, 같은 객체 참조
deep_copy = copy.deepcopy(list1)  # 깊은 복사, 새로운 객체 생성

list1[0] = 'changed'
print(shallow_copy)
print(deep_copy)
