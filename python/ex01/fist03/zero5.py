# 딕셔너리 기반 트라이(Trie) 자료구조 구현
# 문자열의 효율적인 저장과 검색을 위한 트라이 자료구조를 딕셔너리를 사용해 구현
# 단어의 삽입, 검색, 삭제 기능이 있어야 함
# 각 노드는 딕셔너리를 사용해 자식 노드를 관리해야 함
# 트라이 각 노드는 children 딕셔너리를 가지고 있어야 함
# 단어의 끝을 나타내는 플래그(is_end) 사용할 것
class TrieNode:

    def __init__(self):
        # 각 노드는 children 이라는 딕셔너리를 가지고 있음
        # 이 딕셔너리는 현재 노드에서 연결된 자식 노드를 저장
        self.children = {}
        # is_end는 현재 노드가 단어의 끝을 나타내는지 여부를 저장
        # 단어의 끝이라면 True, 아니면 False
        self.is_end = False


class Trie:

    def __init__(self):
        # Trie는 root 노드를 가지고 시작. 루트 노드는 비어있는 상태에서 시작
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        # 단어 삽입을 위해 현재 노드를 루트 노드로 설정
        node = self.root

        # 삽입할 단어의 각 문자에 대해 루프를 돌림
        for char in word:
            # 현재 문자가 자식 노드에 없다면, 새로운 TrieNode를 생성해 추가
            if char not in node.children:
                node.children[char] = TrieNode()
            # 다음 문자를 처리하기 위해 현재 노드를 문자에 해당하는 자식 노드로 변경
            node = node.children[char]

        # 단어의 마지막 문자에 도달했을 때 is_end 플래그 설정
        node.is_end = True

    def search(self, word: str) -> bool:
        # 단어 검색을 위해 현재 노드를 루트 노드로 설정
        node = self.root

        # 검색할 단어의 각 문자에 대해 루프를 돌림
        for char in word:
            # 현재 문자가 자식 노드에 없다면, 이 단어는 트라이에 없는 것임
            if char not in node.children:
                return False

            # 현재 노드를 자식 노드로 이동
            node = node.children[char]

        # 단어의 모든 문자를 확인한 후, 마지막 노드가 단어의 끝인지 확인
        return node.is_end

    def delete(self, word: str) -> None:
        # 내부적으로 재귀 함수를 사용해 단어 삭제
        # 현재 노드를 root로 설정하고, 삭제 과정 시작
        self._delete(self.root, word, 0)

    def _delete(self, node: TrieNode, word: str, index: int) -> bool:
        # 삭제할 단어의 모든 문자를 처리했을 때
        if index == len(word):
            # 현재 노드가 단어의 끝이라면 is_end를 False로 변경
            if not node.is_end:
                return False  # 단어가 존재하지 않음
            node.is_end = False

            # 현재 노드가 더 이상 자식 노드를 가지고 있지 않다면, 삭제 가능
            return len(node.children) == 0

        # 현재 문자에 해당하는 자식 노드를 가져옴
        char = word[index]

        # 현재 문자가 자식 노드에 없다면, 삭제할 단어가 트라이에 없음
        if char not in node.children:
            return False

        # 자식 노드에서 다음 문자로 재귀적으로 삭제 진행
        should_delete_child_node = self._delete(node.children[char], word,
                                                index + 1)

        # 자식 노드를 삭제할 수 있는 경우, 현재 노드의 자식 딕셔너리에서 제거
        if should_delete_child_node:
            del node.children[char]
            # 현재 노드가 더 이상 자식 노드를 가지지 않고, 단어의 끝도 아니라면 이 노드도 삭제 가능
            return len(node.children) == 0
        return False


# Trie 객체 생성
trie = Trie()

# 단어 'apple' 삽입
trie.insert("apple")

# 'apple'이 트라이에 있는지 확인
print(trie.search("apple"))

# 'app'이 트라이에 있는지 확인
print(trie.search("app"))

# 단어 'apple' 삭제
trie.delete("apple")

# 'apple'이 트라이에서 삭제되었는지 확인
print(trie.search("apple"))
