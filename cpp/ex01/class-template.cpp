// 클래스 템플릿
// 여러 타입에 대해 동작할 수 있는 일반화된 클래스를 정의할 수 있게 함
// 특정 데이터 타입에 종속되지 않는 범용 클래스 작성 가능

#include <iostream>

using namespace std;

// T는 템플릿 매개변수, 클래스가 생성될 때 타입이 결정됨
template <typename T>
class MyClass {
   private:
    T data;  // 클래스의 멤버변수 data는 T 타입

   public:
    MyClass(T d) : data(d) {
    }  // End of MyClass Constructor
    T getData() const {
        return data;
    }  // End of getData method
};  // End of MyClass class

// 클래스 템플릿 특수화
template <>
class MyClass<bool> {
   private:
    bool data;

   public:
    MyClass(bool d) : data(d) {
    }
    bool isTrue() const {
        return data;
    }
};

int main(int argc, char const *argv[]) {
    MyClass<int> obj1(10);      // T는 int로 대체됨
    MyClass<double> obj2(5.5);  // T는 double로 대체됨

    cout << obj1.getData() << endl;  // 10
    cout << obj2.getData() << endl;  // 5.5

    return 0;
}
