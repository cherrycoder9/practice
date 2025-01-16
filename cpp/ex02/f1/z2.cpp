#include <iostream>
#include <vector>

int main() {
    using std::cout;
    const std::vector<int> v{1, 2, 3};
    const std::size_t v_size = v.size(); // 벡터의 크기를 저장
    std::cout << v_size << "\n";

    // std::size_t
    // 어떤 타입의 크기도 표현할 수 있을 만큼 충분히 큰 바이트 수를 가지는 부호 없는 정수
    // std::size_t는 항상 벡터의 크기 반환 타입보다 크거나 같음이 보장됨

    // 벡터의 크기 타입은 플랫폼 의존적인 부호 없는 정수 타입으로,
    // std::size_t와 동일할 수도 있고 다를 수도 있음

    // 최근 C++의 auto 및 범위 기반 for 루프 기능 등으로 타입 변환은 큰 문제가 되지 않음
    for (const auto val : v) {
        cout << val << " ";
    }
    return 0;
}