# 문제3: 두 리스트의 교집합 구하기
# 두 개의 리스트를 받아서 공통 요소로 이뤄진 리스트를 반환하는 함수
def find_intersection(list1, list2):
    return list(set(list1) & set(list2))


list1 = [1, 2, 3, 4]
list2 = [3, 4, 5, 6]
result = find_intersection(list1, list2)
print(result)
