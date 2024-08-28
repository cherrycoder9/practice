// Self 참조와 트레이트 객체
// Cache 트레이트 정의하고 get()과 set() 메서드 선언
// get() 메서드는 키를 입력받아 캐시에 저장된 값 반환
// set() 메서드는 키와 값을 받아 캐시에 저장함
// 그런 다음 두 개의 캐시 구현체 작성 InMemoryCache와 FileCache
// InMemoryCache는 데이터를 메모리에 저장하고
// FileCache는 데이터를 파일에 저장함.
// 이 두 캐시를 하나의 트레이트 객체로 사용할 수 있도록 하고
// switch_cache()라는 함수를 작성해 런타임에 캐시 구현체를 바꿀 수 있도록 할것

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
};

// Cache 트레이트 정의
trait Cache {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: String);
}

// InMemoryCache 구조체 정의
// 이 구조체는 데이터를 메모리에 저장하기 위해 HashMap 사용
struct InMemoryCache {
    store: HashMap<String, String>,
}

impl InMemoryCache {
    // InMemoryCache의 새로운 인스턴스를 생성하는 메서드
    fn new() -> Self {
        InMemoryCache {
            // 새로운 빈 HashMap을 생성하는 함수
            store: HashMap::new(),
        }
    }
}

// InMemoryCache에 대해 Cache 트레이트 구현
impl Cache for InMemoryCache {
    // 주어진 키에 해당하는 값을 HashMap에서 검색
    // 값이 있으면 Some(value)를 반환, 없으면 None을 반환
    fn get(&self, key: &str) -> Option<String> {
        // self.store는 InMemoryCache 구조체의 store 필드임
        // HashMap<String, String> 타입
        // get()은 HashMap의 메서드임, 주어진 키에 해당하는 값을 옵셔널 타입인 Option<&V>로 반환
        // 여기서 V는 HashMap의 값 타입, 즉 String 임
        // 키가 있으면 Some(&value)를 반환하고
        // 키가 없으면 None을 반환함
        // cloned()는 Option<&T> 타입에서 사용할 수 있는 메서드인데
        // 이 메서드는 Option 안에 있는 참조 타입의 값을 복제하여 소유권이 있는 값으로 변환해줌
        // Option<&String>에서 Option<String>으로 변환하는 역할을 함
        // HashMap의 get메서드는 키에 대응하는 값에 대한 참조를 반환하기 때문에
        // 참조된 값을 그대로 사용할 경우 소유권 문제가 발생할 수 있음
        // 그래서 cloned 메서드를 사용해 값의 복사본을 생성해 Option<String>으로 반환하는것
        self.store.get(key).cloned()
    }
    fn set(&mut self, key: &str, value: String) {
        // to_string() 메서드는 &str을 String으로 변환함
        // insert는 HashMap의 메서드로, 주어진 키-값 쌍을 삽입함
        // 이 메서드는 키와 값을 HashMap에 저장하고, 만약 동일한 키가 있으면 기존 값을 대체함
        self.store.insert(key.to_string(), value);
    }
}

// FileCache 구조체 정의
struct FileCache {
    file_path: String,
}

impl FileCache {
    // FileCache의 새로운 인스턴스를 생성하는 메서드
    // 파일 경로를 받아서 초기화함
    fn new(file_path: &str) -> Self {
        FileCache {
            file_path: file_path.to_string(),
        }
    }
}

// FileCache에 대해 Cache 트레이트 구현
impl Cache for FileCache {
    // 파일에서 주어진 키에 해당하는 값 검색
    // 키가 존재하면 Some(value) 반환, 없으면 None 반환
    fn get(&self, key: &str) -> Option<String> {
        // 파일을 염. 파일이 없으면 None 반환
        let mut file = match File::open(&self.file_path) {
            // Result<T, E> 타입에서 성공적인 결과를 나타내는 패턴
            // f는 Ok 변형에 담긴 값(T)을 의미함. Ok 안에 들어있는 실제 값이 f로 바인딩됨
            Ok(f) => f,
            // Result<T, E> 타입에서 오류를 나타내는 패턴
            // _는 와일드카드 패턴으로, 어떤 값이 와도 상관없다는 의미
            // 오류가 발생했지만 그 오류 값이 무엇인지는 중요하지 않기 때문에 _로 처리
            Err(_) => return None,
        };

        let mut contents = String::new();
        // 파일 내용을 문자열로 읽어옴
        // read_to_string: 성공하면 읽어들인 문자열을 String에 저장하고 읽은 바이트 수 반환
        // 실패하면 Err를 반환
        // is_ok: Result가 Ok일 경우 true를 반환하고 Err일 경우 false를 반환
        if file.read_to_string(&mut contents).is_ok() {
            // 각 라인을 순회하면서 키를 검색함
            // lines: 문자열을 줄 단위로 나눠 각 줄을 차례로 이터레이션할 수 있게 해줌
            for line in contents.lines() {
                // splitn: 문자열을 주어진 구분자로 최대 n번 나눔
                // 결과는 이터레이터로 반환되어 각 조각 순회 가능
                // collect: 이터레이터의 모든 요소를 모아 하나의 컬렉션으로 만듬
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                // 키가 일치하면 해당 값을 반환함
                if parts.len() == 2 && parts[0] == key {
                    return Some(parts[1].to_string());
                }
            }
        }
        None
    }

    // 파일에 주어진 키와 값을 저장함
    fn set(&mut self, key: &str, value: String) {
        // 기존 파일 내용을 읽어옴. 파일이 없으면 빈 문자열로 시작함
        let mut contents = match fs::read_to_string(&self.file_path) {
            Ok(c) => c,
            Err(_) => String::new(),
        };

        let mut found = false;
        // 각 라인을 순회하면서 키를 검색하고, 키가 있으면 값을 업데이트 함
        contents = contents
            .lines()
            // map: 이터레이터의 각 요소를 순회하면서 클로저 또는 함수로 변환함
            // |line|과 같은 표현은 클로저(익명 함수)를 정의하는 방식임
            // |line|은 클로저의 인자를 나타내고, 뒤에 오는 {} 또는 식에서 이 인자를 사용해 작업을 수행
            // 예: |x| x * 2 (x를 받아서 2배로 만든 값을 반환)
            .map(|line| {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 && parts[0] == key {
                    found = true;
                    // format!()은 문자열을 생성하는 매크로로, 다른 언어의 sprintf
                    // String.format() 등과 비슷함. 주어진 형식에 따라 값을 삽입해 새로운 문자열 반환
                    format!("{}={}", key, value)
                } else {
                    line.to_string()
                }
            })
            // _는 타입을 추론할 수 있게 하는 자리표시자로, 러스트가 해당 타입을 자동으로 결정함
            // <Vec<_>>는 colect() 메서드와 함께 사용되어 이터레이터의 결과를 벡터로 수집함
            .collect::<Vec<_>>()
            .join("\n");

        // 키가 파일에 없으면, 새로운 키-값 쌍을 추가함
        if !found {
            // push_str: String 타입에 다른 문자열을 추가하는 메서드
            // 문자열을 원본에 덧붙이는 방식으로 작동함
            contents.push_str(&format!("\n{}={}", key, value));
        }

        // 파일에 업데이트된 내용을 다시 씀
        // expect: Result 또는 Option 타입에서 사용되며, Ok나 Some값을 기대할 때 사용
        // 값이 없거나 오류가 발생한 경우, expect()는 프로그램을 종료시키고 주어진 오류 메시지 출력
        let mut file = File::create(&self.file_path).expect("파일 생성 실패");
        // as_bytes(): String이나 &str 타입을 바이트 슬라이스(&[u8])로 변환하는 메서드
        // 문자열을 바이트 배열로 처리할 때 유용함
        // 문자열을 바이트 단위로 접근하거나 처리할 수 있게 해줌
        file.write_all(contents.as_bytes()).expect("파일 쓰기 실패");
    }
}

// 런타임에 캐시 구현체를 교체하는 함수
// cureent_cache의 값을 new_cache로 변경
// dyn은 러스트에서 트레이트 객체를 사용하기 위해 도입된 키워드
// 런타임에 특정 트레이트를 구현한 객체에 대한 동적 디스패치를 가능하게 해줌
fn switch_cache(current_cache: &mut Box<dyn Cache>, new_cache: Box<dyn Cache>) {
    *current_cache = new_cache;
}

pub fn main() {
    // InMemoryCache를 사용해 데이터를 저장하고 검색
    let mut cache: Box<dyn Cache> = Box::new(InMemoryCache::new());
    cache.set("key1", "value1".to_string());
    println!("InMemoryCache - key1: {:?}", cache.get("key1"));

    // FileCache로 변경하고데이터를 저장하고 검색함
    let file_cache_path = "cache.txt";
    let mut file_cache: Box<dyn Cache> = Box::new(FileCache::new(file_cache_path));
    file_cache.set("key2", "value2".to_string());
    println!("FileCache - key2: {:?}", file_cache.get("key2"));

    // InMemoryCache에서 FileCache로 캐시를 교체하고 데이터 검색
    switch_cache(&mut cache, file_cache);
    println!("캐시 스위치됨 - key2: {:?}", cache.get("key2"));
}
