// 유저 데이터와 관련된 함수들을 제공하는 모듈
// 유저 정보를 저장하고 검색하는 함수 정의

// 'export' 키워드는 해당 변수, 함수, 클래스 등을 외부 파일에서 사용할 수 있도록 내보내는 역할 
export interface User {
    id: number; // 유저 고유 번호
    name: string; // 유저 이름
    email: string; // 유저 이메일 주소
}

// 유저 데이터를 저장할 배열 (실제 환경에선 DB)
let users: User[] = [];

// 새로운 유저를 추가하는 함수
export function addUser(user: User): void {
    users.push(user);
    console.log(`${user.name} 유저가 추가되었습니다.`);
}

// 유저 ID를 통해 유저 정보를 검색하는 함수
export function getUserById(id: number): User | undefined {
    return users.find(user => user.id === id);
}

// 모든 유저 리스트를 반환하는 함수
export function getAllUsers(): User[] {
    return users;
}