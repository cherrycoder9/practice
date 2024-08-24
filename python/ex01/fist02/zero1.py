# 리스트 컴프리헨션
# 리스트를 간결하게 생성하는 방법
# 0부터 9까지의 제곱수를 가진 리스트 생성
# for는 반복문을 시작할때 사용하는 키워드
# for 뒤에 오는 변수는 각 반복 단계에서 시퀀스의 요소를 받음
# in은 반복문을 수행할 대상(시퀀스)을 지정할 때 사용하는 키워드
# in 뒤에는 리스트, 튜플, 문자열, range객체 등 반복가능한(iterable) 객체가 올수있음
squares = [x**2 for x in range(10)]
print(squares)

# 중첩 리스트 컴프리헨션
# 3x3 행렬 생성
matrix = [[i * j for j in range(3)] for i in range(3)]
print(matrix)

# 리스트 슬라이싱
# 리스트의 일부를 추출
my_list = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
sub_list = my_list[2:7]  # 인덱스 2부터 6까지 추출
print(sub_list)

# 리스트 뒤집기
reversed_list = my_list[::-1]
print(reversed_list)

# 리스트 메소드
# 리스트에 대한 다양한 메소드를 활용해 리스트를 조작할 수 있음
my_list = [1, 2, 3]

# 요소 추가
my_list.append(4)
print(my_list)

# 요소 삽입
my_list.insert(1, 'a')
print(my_list)

# 요소 제거
my_list.remove('a')
print(my_list)

# 요소의 인덱스 찾기
index_of_3 = my_list.index(3)
print(index_of_3)

# 요소 개수 세기
count_of_2 = my_list.count(2)
print(count_of_2)

# 리스트 확장, 모든 요소들을 리스트의 끝에 개별적으로 추가한다는 점이 append와 다른 점
my_list.extend([5, 6])
print(my_list)
