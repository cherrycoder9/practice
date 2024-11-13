// 비동기 함수와 Promise (async/await)

// 외부 API 호출을 시뮬레이션하는 함수 정의
// 네트워크 요청을 모방하기 위해 Promise와 setTimeout 사용
// Promise는 자바스크립트에서 비동기 작업의 완료나 실패를 나타내는 객체임
// 비동기 작업이 끝나면 resolve나 reject로 결과를 반환함
// TS에서는 Promise가 반환하는 결과의 타입을 명시할 수 있는데 그게 바로 제네릭 < > 사이에 있는 부분 
const fetchUserData = async (userId: number): Promise<{ id: number; name: string; age: number; }> => {
    return new Promise((resolve) => {
        setTimeout(() => {
            resolve({ id: userId, name: `User${userId}`, age: 30 + userId });
        }, 1000); // 1초 후에 데이터 반환 
    });
};

// 다중 비동기 작업 처리
// 여러 사용자의 데이터를 병렬로 가져오는 함수
// Promise.all()
const fetchMultipleUsers = async (userIds: number[]): Promise<void> => {
    try {
        // 여러 사용자 ID를 받아와 각각의 데이터를 동시에 요청함 
        const userPromises = userIds.map((id) => fetchUserData(id));
        const users = await Promise.all(userPromises);

        // 각 사용자 정보를 콘솔에 출력
        console.log(`모든 사용자 데이터:`);
        users.forEach((user) => {
            console.log(`사용자 ID: ${user.id}, 이름: ${user.name}, 나이: ${user.age}`);
        });
    } catch (error) {
        console.error(`사용자 데이터 로딩중 오류 발생: ${error}`);
    }
};

// 비동기 흐름 제어 - 순차 처리
// 사용자 데이터를 순차적으로 처리하는 함수
const processUsersSequentially = async (userIds: number[]): Promise<void> => {
    try {
        for (const userId of userIds) {
            // 각 사용자 데이터를 순차적으로 가져와 처리함
            const user = await fetchUserData(userId);
            console.log(`ID: ${user.id}, 이름: ${user.name}, 나이: ${user.age}`);
        }
    } catch (error) {
        console.error(`순차적으로 사용자 데이터를 처리하는 중 오류 발생: ${error}`);
    }
};

// 비동기 지연 처리
// 특정 시간만큼 지연 후에 작업을 수행하는 함수
const delay = (ms: number): Promise<void> => {
    return new Promise((resolve) => setTimeout(resolve, ms));
};

// 비동기 지연을 이용한 사용자 데이터 처리
const fetchWithDelay = async (userId: number): Promise<void> => {
    try {
        console.log(`사용자 ${userId} 데이터를 가져오기 전 대기 중...`);
        await delay(2000); // 2초 대기 
        const user = await fetchUserData(userId);
        console.log(`지연 후 가져온 사용자 ID: ${user.id}, 이름: ${user.name}, 나이: ${user.age}`);
    } catch (error) {
        console.error(`지연된 사용자 데이터 로딩 중 오류 발생: ${error}`);
    }
};

// 예제 함수 호출부
(async () => {
    console.log(`=== 병렬 처리 예제 ===`);
    await fetchMultipleUsers([1, 2, 3]);

    console.log(`=== 순차 처리 예제 ===`);
    await processUsersSequentially([4, 5, 6]);

    console.log(`=== 지연 처리 예제 ===`);
    await fetchWithDelay(7);
})();