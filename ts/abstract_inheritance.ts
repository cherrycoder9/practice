// 추상 클래스와 상속
// 추상 클래스 Animal, 다른 클래스에서 상속받아 사용하기 위해 공통 동작과 속성 정의
abstract class Animal2 {
    protected name: string;

    constructor(name: string) {
        this.name = name;
    }

    // 추상 메서드, 서브 클래스에서 반드시 구현해야 함 
    abstract makeSound(): void;

    // 공통 메서드, 모든 동물에게 적용되는 일반적 동작 정의
    move(): void {
        console.log(`${this.name} 이동한다.`);
    }
}

// Dog 클래스, Animal 클래스를 상속받아 구현한 구체 클래스
class Dog2 extends Animal2 {
    constructor(name: string) {
        super(name); // 부모 클래스의 생성자를 호출해 name 초기화 
    }

    // 추상 메서드 makeSound() 구현, 각 동물에 맞는 소리 정의
    makeSound(): void {
        console.log(`${this.name} 멍멍 짖는다.`);
    }

    // 추가적인 메서드, Dog 클래스에 특화된 동작 정의
    fetch(): void {
        console.log(`${this.name} 공을 물어온다.`);
    }
}

// Cat 클래스
class Cat2 extends Animal2 {
    constructor(name: string) {
        super(name);
    }

    // 추상 메서드 구현
    makeSound(): void {
        console.log(`${this.name} 야옹하고 운다`);
    }

    scratch(): void {
        console.log(`${this.name} 발톱을 세운다.`);
    }
}

// Zoo 클래스, 여러 Animal 객체를 관리하기 위한 클래스
class Zoo {
    private animals: Animal2[] = [];

    // 동물을 동물원에 추가하는 메서드
    addAnimal(animal: Animal2): void {
        this.animals.push(animal);
        console.log(`${animal.constructor.name} 동물원 추가됨`);
    }

    // 동물원에 있는 모든 동물의 소리를 내게 하는 메서드
    makeAllSounds(): void {
        this.animals.forEach((animal) => animal.makeSound());
    }

    // 모든 동물이 움직이게 하는 메서드
    moveAllAnimals(): void {
        this.animals.forEach((animal) => animal.move());
    }
}

// 실제 동물 객체 생성 및 사용
const myDog2 = new Dog2('바둑이');
const myCat2 = new Cat2('나비');

const myZoo = new Zoo();
myZoo.addAnimal(myDog2);
myZoo.addAnimal(myCat2);

myZoo.makeAllSounds();
myZoo.moveAllAnimals();
myDog2.fetch();
myCat2.scratch();