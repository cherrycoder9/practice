#include <iostream>
#include <vector>

int main() {
    using namespace std;

    // C++17에서 도입된 클래스 템플릿 인수 유추는 auto와 유사하게
    // 초기화되는 데이터에 따라 템플릿 매개변수의 타입을 자동으로 유추함
    vector<int> v1{1, 2, 3};
    vector v2{1, 2, 3};

    return 0;
}
