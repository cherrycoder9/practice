struct Data {
    int value;  
};

class BaseDataProvider {
public:
    BaseDataProvider() = default;
    BaseDataProvider(const BaseDataProvider&) = delete; // 복사 생성자
    BaseDataProvider(BaseDataProvider&&) = default; // 이동 생성자 
    BaseDataProvider& operator=(const BaseDataProvider&) = delete;
    BaseDataProvider& operator=(BaseDataProvider&&) = default;
    virtual ~BaseDataProvider() = default;

    virtual Data getData() const = 0;
    // virtual -> 가상함수: 파생 클래스에서 재정의할 수 있도록 함
    // Data -> 함수의 반환 타입 
    // getData() -> 함수의 이름 
    // const -> 멤버 변수를 변경하지 않는 함수임을 나타냄
    // = 0 -> 순수 가상함수, 이 함수는 구현이 없으며, 반드시 파생 클래스에서 구현해야 함을 의미
};

// default는 컴파일러가 기본 구현을 자동으로 생성하도록 지시하는 키워드
// 기본생성자를 명시적으로 지정하는 코드임
// delete는 특정 기능을 사용하지 못하게 막는 역할
// 해당 함수(생성자, 연산자 등)의 사용을 금지함 

// void* operator new(size_t) = delete; // new 연산자 사용 금지
// void operator delete(void*) = delete; // delete 연산자 사용 금지
// operator int() = delete;  // int 타입으로 변환 금지