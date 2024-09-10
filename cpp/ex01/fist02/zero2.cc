// 사용자 정의 자료형
// 구조체, 공용체, 열거형 등을 사용해 새로운 자료형을 정의함
// 구조체 (struct): 여러 다른 자료형을 하나의 자료형으로 묶음

#include <iostream>
using namespace std;

struct Person {
    string name;
    int age;
    float height;
};

int main(int argc, char const *argv[]) {
    /// 구조체 변수 선언 및 초기화
    Person person1 = {"John", 25, 175.5};

    // 구조체 멤버에 접근
    cout << "이름: " << person1.name << endl;
    cout << "나이: " << person1.age << endl;
    cout << "키: " << person1.height << endl;
    return 0;
}
