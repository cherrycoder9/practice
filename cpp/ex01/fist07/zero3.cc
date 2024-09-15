// 여러가지 타입의 데이터를 출력하는 print 함수를 구현
// int, double, string 타입의 매개변수를 처리할 수 있어야 함
// 두번째 매개변수로 줄바꿈을 제어할 수 있어야 함

#include <iostream>
#include <string>

using namespace std;

void print(int value, bool newline = true) {
    cout << value;

    if (newline) {
        cout << endl;
    }
}

void print(double value, bool newline = true) {
    cout << value;

    if (newline) {
        cout << endl;
    }
}

void print(const string& value, bool newline = true) {
    cout << value;

    if (newline) {
        cout << endl;
    }
}

int main() {
    print(10);
    print(3.14, false);
    print("Hello, World!", true);

    return 0;
}