# 리스트에서 특정 요소 제거하기 문제
# 특정 요소를 제거하는 함수를 작성하기
# 인수는 lst, value 두개
# 주어진 value 값을 리스트 lst에서 모두 제거한 새로운 리스트를 반환해야 함
import copy

# def remove_all_occurrences(lst, value):
#     list2 = copy.deepcopy(lst)
#     while True:
#         if value in list2:
#             list2.remove(value)
#         else:
#             break
#     return list2


# 위 방법 대신에 리스트 컴프리헨션을 이용하면 요소가 많을때 성능을 높일 수 있음
def remove_all_occurrences(lst, value):
    return [item for item in lst if item != value]


list1 = [1, 2, 3, 2, 4, 2]
list2 = remove_all_occurrences(list1, 2)
print(list1)
print(list2)
