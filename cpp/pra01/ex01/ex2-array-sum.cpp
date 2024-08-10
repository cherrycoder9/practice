// 배열 합산
// 함수 템플릿을 사용해 배열과 그 크기를 인자로 받아
// 배열 내의 모든 원소의 합을 반환하는 함수.
// 정수형 배열, 부동소수점 배열 등 여러
// 타입의 배열에서 작동해야 함.

#include <iostream>

using namespace std;

template <typename T>
T arraySum(T* array, size_t size) {
    // T sum = 0;
    // string 같은 타입에 대해서도 적용하려면 아래처럼 작성
    // T 타입에 맞는 기본값으로 sum을 초기화함
    // int, long, short, char : 0
    // float, double, long double : 0.0
    // bool : false
    // T*(포인터형) : nullptr
    // 사용자 정의 자료형(클래스, 구조체 등) : 기본 생성자를 호출함
    // STL 컨테이너 : 빈 컨테이너로 초기화
    T sum = T();
    for (size_t i = 0; i < size; i++) {
        sum += array[i];
    }
    return sum;
}

int main(int argc, char const* argv[]) {
    int arr1[] = {1, 2, 3, 4, 5};
    double arr2[] = {1.1, 2.2, 3.3};
    string arr3[] = {string("ab"), string("cd")};

    int sum1 = arraySum(arr1, 5);
    double sum2 = arraySum(arr2, 3);
    string sum3 = arraySum(arr3, 2);

    cout << sum1 << endl;
    cout << sum2 << endl;
    cout << sum3 << endl;

    return 0;
}
