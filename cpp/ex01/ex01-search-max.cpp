// 최대값 찾기
// 함수 템플릿 사용해 두 개의 인자를 받아 그 중 더 큰 값을 반환하는 함수 작성
// 정수, 부동소수점수, 문자열에 대해서 작동해야 함

#include <iostream>

using namespace std;

// 파일명 한글로 하면 컴파일시 에러남
// 함수 이름을 max로 하면 std::max 라이브러리 함수랑 충돌함
template <typename T>
T myMax(T a, T b) {
    return (a > b) ? a : b;
}  // End of max method

int main(int argc, char const* argv[]) {
    int a = myMax(10, 20);
    cout << a << endl;
    double b = myMax(10.5, 20.5);
    cout << b << endl;
    string c = myMax(string("apple"), string("orange"));
    cout << c << endl;
    return 0;
}

// "apple", "orange" 같은 문자열 리터럴은 const char* 타입으로 간주됨
// 따라서 myMax 함수를 호출할 때 const char* 타입을 인자로 받게 됨
// const char*에 대한 비교는 문자열 자체의 사전순 비교를 하지 않고
// 포인터 주소값을 비교하게 됨
// 해결하려면 std::string 객체로 명시적으로 변환해 주거나
// const char* 타입에 대한 특수화 템플릿을 만들어야 함(리턴시 strcmp 사용)