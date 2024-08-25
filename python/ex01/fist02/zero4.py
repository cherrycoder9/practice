# 문제: 리스트 중첩 구조 평탄화
# 중첩된 리스트를 받아 평탄화하는 함수 작성
# 중첩된 모든 리스트를 1차원 리스트로 반환
def flatten_list(nested_list):
    flat_list = []
    for item in nested_list:
        # isinstance()는 파이썬 내장 함수
        # 특정 객체가 지정된 클래스나 데이터 타입과 일치하는지 확인할때 사용함
        if isinstance(item, list):
            # 재귀 호출로 중첩 리스트 평탄화
            flat_list.extend(flatten_list(item))
        else:
            flat_list.append(item)
    return flat_list


list1 = [1, [2, 3], [4, [5, 6]], 7]
list2 = flatten_list(list1)
print(list1)
print(list2)
