#include <complex>
#include <iostream>
#include <map>
#include <vector>

int main() {
    using std::vector, std::string, std::cout, std::map, std::complex;

    // 범위 기반 for 루프에서 참조 연산자 &를 사용하면 벡터 요소 직접 수정
    vector v{1.0, 2.0, 3.0, 4.0, 5.0};
    for (double &x: v) {
        x *= x; // 각 요소 제곱
    }

    // 불변 데이터 순회시엔 const 참조를 사용하는게 효율적
    vector<string> sorry_dave{"Open", "the", "pod", "bay", "doors", "HAL"};
    for (const string &s: sorry_dave) {
        cout << s << " ";
    }

    // C++11 이전에는 긴 템플릿 타입의 가독성을 높이기 위해 typedef를 사용하는 방법이 있었음
    // 복잡한 맵 타입을 별칭으로 정의하는 방법은 아래와 같았음
    typedef map<string, complex<double>> complex_map;

    // C++11 이후 using 문은 같은 작업을 더 자연스러운 문법으로 수행할 수 있도록 확장됨
    // using은 고급 템플릿 기법에서도 더 유연하게 동작함
    using complex_map2 = map<string, complex<double>>;

    // C++11에서 도입된 통합 초기화는 중괄호를 사용해 객체를 초기화하는 새로운 방법
    int i{100}; // int i = 100; 과 같음
    // 통합 초기화는 좁혀지는 변환(narrowing conversion)을 방지함
    // 기존 초기화 방식에서는 아래와 같은 코드가 경고 없이 컴파일 될 수 있었음
    constexpr double x = 92.09;
    int k = x;
    // int n{x}; // 컴파일 에러

    // 벡터에서 요소 개수 초기화
    // 벡터의 크기를 명시적으로 지정하려면 소괄호 초기화를 사용하면 됨
    vector<int> vec(2); // 크기가 2인 벡터 생성

    return 0;
}