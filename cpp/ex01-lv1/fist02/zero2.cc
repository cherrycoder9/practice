// ����� ���� �ڷ���
// ����ü, ����ü, ������ ���� ����� ���ο� �ڷ����� ������
// ����ü (struct): ���� �ٸ� �ڷ����� �ϳ��� �ڷ������� ����

#include <iostream>
using namespace std;

struct Person {
    string name;
    int age;
    float height;
};

int main(int argc, char const *argv[]) {
    // ����ü ���� ���� �� �ʱ�ȭ
    Person person1 = {"John", 25, 175.5};

    // ����ü ����� ����
    cout << "�̸�: " << person1.name << endl;
    cout << "����: " << person1.age << endl;
    cout << "Ű: " << person1.height << endl;
    return 0;
}
