# 중첩 딕셔너리
# 딕셔너리는 값으로 다른 딕셔너리를 포함할 수 있음
nested_dict = {
    "student1": {
        "name": "김박사",
        "age": 28
    },
    "student2": {
        "name": "이석사",
        "age": 25
    }
}

# 중첩된 값 조회
print(nested_dict["student1"]["name"])  # 김박사
print(nested_dict["student2"]["age"])  # 25

# 딕셔너리는 내부적으로 해시 테이블을 사용해 키를 관리함
# 값을 빠르게 조회할 수 있음, 하지만 해시 충돌 발생 가능성 있음
# 충돌 처리 방법엔 체이닝, 오픈 어드레싱 등이 있음. 파이썬 딕셔너리는 체이닝을 사용함

# 파이썬 3.9부터는 딕셔너리 병합 및 업데이트를 위한 새로운 연산자 추가
# 딕셔너리 병합
dict1 = {"a": 1, "b": 2}
dict2 = {"b": 3, "c": 4}

# | 연산자를 사용한 병합
merged_dict = dict1 | dict2
print(merged_dict)  # {'a': 1, 'b': 3, 'c': 4}

# |= 연산자를 사용한 병합 후 업데이트
dict1 |= dict2
print(dict1)  # {'a': 1, 'b': 3, 'c': 4}

# 딕셔너리와 유사한 데이터 클래스
# 파이썬 3.7 이후로 특정 데이터를 가지는 클래스 구조인 데이터 클래스 사용 가능
# 복잡한 데이터 구조에 대해 더 나은 타이팅 및 코드 구조화

from dataclasses import dataclass


@dataclass
class Student:
    name: str
    age: int


student = Student(name="김철수", age=30)
print(student.name)  # 김철수
print(student.age)  # 30
