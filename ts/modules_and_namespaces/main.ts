// import 키워드는 다른 파일에서 export된 함수나 변수를 가져와 사용하는 역할
import { addUser, getAllUsers, getUserById, User } from './example_domule';

// 새로운 유저 추가
const user1: User = { id: 1, name: '홍길동', email: 'hong@example.com' };
const user2: User = { id: 2, name: '김철수', email: 'kim@example.com' };
addUser(user1);
addUser(user2);

// 특정 유저 검색
const foundUser = getUserById(1);
if (foundUser) {
    console.log(`검색된 유저 이름: ${foundUser.name}, 이메일: ${foundUser.email}`);
} else {
    console.log(`해당 ID의 유저를 찾을 수 없습니다.`);
}

// 모든 유저 출력
console.log(`모든 유저 목록:`, getAllUsers());
