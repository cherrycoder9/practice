// 제네릭 스택 구현
// 템플릿 클래스를 사용해 임의의 타입을
// 저장할 수 있는 스택 클래스 구현
// push, pop, top, isEmpty 메서드 구현

#include <iostream>

using namespace std;

template <typename T>
class Stack {
   private:
    vector<T> elements;

   public:
    // 참조자 &를 붙인 이유는, 전달된 인자를 복사하지 않고
    // 원래 객체를 참조하기 위해 사용함.
    // 불필요한 복사가 발생하지 않으므로 성능 향상.
    // const: 참조된 객체가 함수 내부에서 수정되지 않도록 보장
    // 전달된 객체를 수정하지 않으면서 참조를 통해 접근
    void push(T const& elem) {

    };
    void pop() {

    };
    T top() const {

    };
    bool isEmpty() const {

    };
};

int main(int argc, char const* argv[]) {
    /* code */
    return 0;
}
