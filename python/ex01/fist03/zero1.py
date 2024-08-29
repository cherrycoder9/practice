# 딕셔너리: 키-값 쌍으로 데이터를 저장하는 해시 테이블 기반 구조
# 각 키는 유일하며, 해당 키에 대응하는 값을 저장함
# 매우 빠른 조회 속도, 효율적 데이터 관리 가능

# 딕셔너리 생성
person = {"name": "김박사", "age": 25, "city": "포항"}

# 값 조회
print(person["name"])  # 김박사

# 값 추가 또는 업데이트
person["job"] = "Engineer"
print(person)

# 키가 존재하는지 확인
if "age" in person:
    print("person에 age 있음")

# 값 삭제
del person["city"]
print(person)  # {'name': '김박사', 'age': 25, 'job': 'Engineer'}

# 딕셔너리 주요 메서드
# keys(), values(), items() 메서드
print(person.keys())  # dict_keys(['name', 'age', 'job'])
print(person.values())  # dict_values(['김박사', 25, 'Engineer'])
print(person.items())  # dict_items([('name', '김박사'), ('age', 25), ...

# get() 메서드: 키가 존재하지 않을 때 디폴트 값 반환
print(person.get("city", "키 없음"))  # 키 없음

# pop() 메서드: 키에 해당하는 값 반환하며 딕셔너리에서 제거
job = person.pop("job")
print(job)  # Engineer
print(person)  # {'name': '김박사', 'age': 25}

# popitem() 메서드: 마지막 키-값 쌍을 반환하며 제거
name = person.popitem()
print(name)  # ('age', 25)
print(person)  # {'name': '김박사'}

# clear() 메서드: 모든 항목을 제거
person.clear()
print(person)  # {}

# 딕셔너리 컴프리헨션
# 짧고 간결하게 딕셔너리를 생성하는 방법
squares = {x: x**2 for x in range(4)}
print(squares)  # {0: 0, 1: 1, 2: 4, 3: 9}
