# 딕셔너리 병합 및 중첩된 키 업데이트
# dict1과 dict2를 병합
# 중첩된 딕셔너리 키가 동일할 경우 dict2의 값을 사용해 업데이트
# 딕셔너리가 중첩된 경우에도 병합이 올바르게 이뤄져야 함
# 재귀적인 방법으로 문제 해결, 병합 연산자 사용 금지
def deep_merge(dict1: dict, dict2: dict) -> dict:
    merged = dict1.copy()  # dict1의 복사본 생성
    for key, value in dict2.items():
        if key in merged and isinstance(merged[key], dict) and isinstance(
                value, dict):
            # 만약 키가 양쪽 딕셔너리에 모두 존재하고, 값이 딕셔너리라면 재귀적으로 병합
            merged[key] = deep_merge(merged[key], value)
        else:
            # 그렇지 않다면 dict2의 값을 사용해 덮어쓰기
            merged[key] = value
    return merged


# 테스트 예시
dict1 = {"a": 1, "b": {"x": 2, "y": 3}, "c": 4}
dict2 = {"b": {"y": 30, "z": 40}, "c": 5, "d": 6}

result = deep_merge(dict1, dict2)
print(result)  # {'a': 1, 'b': {'x': 2, 'y': 30, 'z': 40}, 'c': 5, 'd': 6}
