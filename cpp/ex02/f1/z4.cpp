#include <format>
#include <iostream>

int main() {
    using std::cout;
    using std::format;
    using std::string;

    // C++20에서 새롭게 도입된 표준 라이브러리 중 std::format 함수가 있음
    // 기존에는 체이닝 방식으로 출력해 코드가 길어지고 가독성이 떨어짐
    // std::cout << "u = " << u << ", v = " << v << "\n";
    // format 함수는 문자와 숫자 데이터를 텍스트 형식으로 변환해 문자열 객체로
    // 반환 가능함

    constexpr double u = 1.5;
    constexpr double v = 4.2;
    // 값이 왼쪽에서 오른쪽 순서대로 입력된다면, {}처럼 인덱스를 생략해도 됨.
    // 값을 재사용하거나 순서가 고정되어야 하는 경우 인덱스를 사용
    const string output = format("u = {0}, v = {1}\n", u, v);
    cout << output;
}
