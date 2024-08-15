// 공용체 (union)
// 여러 자료형을 동일한 메모리 위치에 저장할 수 있음

#include <iostream>
using namespace std;

union Data {
    int intValue;
    float floatValue;
    char charValue;
};

int main(int argc, char const *argv[]) {
    Data data;

    data.intValue = 10;
    cout << "정수값: " << data.intValue << endl;

    data.floatValue = 3.14;
    cout << "실수값: " << data.floatValue << endl;

    data.charValue = 'A';
    cout << "문자값: " << data.charValue << endl;

    cout << "정수값: " << data.intValue << endl;
    cout << "실수값: " << data.floatValue << endl;
    cout << "문자값: " << data.charValue << endl;

    // 공용체에서는 가장 마지막에 저장된 값만 정확히 유지됨
    // 마지막 출력에서 intValue, floatValue는 유효하지 않을 수 있음
    return 0;
}
