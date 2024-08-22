# 가변 변수 타입으로 인해 예상하지 못한 동작 발생 가능성
# 함수의 기본 인자로 가변 객체를 사용할 때 특히 주의


# 가변 객체를 기본 인자로 사용하지 말 것
# 파이썬의 인자 처리 방식 때문
# 기본 인자는 함수 정의 시점에 한번만 평가됨
# 따라서 이 리스트는 함수가 여러번 호출되더라도 같은 리스트 객체를 참조하게 됨
#
def append_to_list(value, lst=[]):
    lst.append(value)
    return lst


list1 = append_to_list(1)
list2 = append_to_list(2)
print(list1)  # [1, 2]
print(list2)  # [1, 2]


# 올바른 방법
# None을 기본값으로 사용하고, 내부에서 새로운 리스트를 생성
def append_to_list_properly(value, lst=None):
    if lst is None:  # lst가 None이면 새로운 리스트를 생성
        lst = []
        lst.append(value)
        return lst


list3 = append_to_list_properly(1)
list4 = append_to_list_properly(2)

print(list3)  # [1]
print(list4)  # [2]
