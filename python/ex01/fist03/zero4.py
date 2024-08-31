# 커스텀 해시맵 클래스 구현
# 고유한 해시 함수를 사용해 커스텀 해시맵 클래스 구현
# 이 클래스는 파이썬 딕셔너리와 유사한 기능을 갖췄지만,
# 해시 충돌이 발생할 경우 체이닝 대신 오픈 어드레싱(선형 탐사)를 사용
# 초기 해시맵 크기는 8로 설정하고, 요소가 85% 이상 채워지면 두배로
# 기본 파이썬 딕셔너리를 사용하지 말고 해시충돌처리 직접 구현
class CustomHashMap:

    def __init__(self):
        self.size = 8  # 초기 해시맵 크기
        self.count = 0  # 현재 요소의 개수
        self.map = [None] * self.size

    # 해시 함수
    # 파이썬에서 함수나 변수 이름 앞에 '_'를 붙이는 이유
    # 해당 함수나 변수가 비공개 또는 내부 용도로 사용됨을 의미하는 관례
    # 클래스 외부에서 직접 접근이 되지 않음을 의도하기 위함
    def _hash(self, key: str) -> int:
        # 간단한 해시 함수 (문자열의 각 문자의 유니코드 값 합산)
        # ord() 함수는 파이썬 내장 함수
        # 단일 문자의 유니코드 코드를 반환함
        # 반대로 chr() 함수는 유니코드 값을 입력받아 해당하는 문자 반환
        return sum(ord(char) for char in key) % self.size

    # 크기 조정
    # self는 메서드가 호출된 객체 자신을 참조함
    def _resize(self):
        # 해시맵의 크기를 두 배로 늘리고, 기존 데이터를 다시 해싱
        old_map = self.map
        self.size *= 2
        self.map = [None] * self.size
        self.count = 0

        for item in old_map:
            if item is not None:
                self.put(item[0], item[1])

    # 삽입
    def put(self, key: str, value: int) -> None:
        if self.count / self.size >= 0.75:
            self._resize()

        # 주어진 키를 해시 함수에 전달해 해시 값 계산하고 인덱스 결정
        index = self._hash(key)
        # 원래 시작 위치를 추적하는데 사용할 변수
        original_index = index
        # 오픈 어드레싱 방식에서 선형 탐사를 위한 스텝 크기
        # 여기서는 한 칸씩 이동하며 탐색 진행
        probing = 1

        while self.map[index] is not None:
            # 이미 동일한 키가 있는 경우 값 업데이트
            if self.map[index][0] == key:
                self.map[index] = (key, value)
                return
            # 오픈 어드레싱: 선형 탐사
            index = (original_index + probing) % self.size
            probing += 1

        # 새로운 키-값 삽입
        self.map[index] = (key, value)
        self.count += 1

    # 조회
    def get(self, key: str) -> int:
        index = self._hash(key)
        original_index = index
        probing = 1

        while self.map[index] is not None:
            if self.map[index][0] == key:
                return self.map[index][1]
            index = (original_index + probing) % self.size
            probing += 1

        # 키가 존재하지 않는 경우
        return None

    # 삭제
    def delete(self, key: str) -> None:
        index = self._hash(key)
        original_index = index
        probing = 1

        while self.map[index] is not None:
            if self.map[index][0] == key:
                self.map[index] = None
                self.count -= 1
                # 삭제 후 재 해싱으로 빈 슬롯을 없애야 할 수 있음
                return
            index = (original_index + probing) % self.size
            probing += 1

    def _rehash_after_deletion(self, start_index: int) -> None:
        index = (start_index + 1) % self.size

        while self.map[index] is not None:
            key, value = self.map[index]
            self.map[index] = None
            self.count -= 1
            self.put(key, value)  # 재 해싱
            index = (index + 1) % self.size


map = CustomHashMap()
map.put("사과", 1)
map.put("바나나", 2)
map.put("포도", 3)

print(map.get("사과"))  # 1
print(map.get("바나나"))  # 2

map.delete("바나나")
print(map.get("바나나"))  # None

map.put("체리", 4)
print(map.get("체리"))  # 4
