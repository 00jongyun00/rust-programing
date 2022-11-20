#![allow(unused_variables)] // idea 를 실험하는 동안 compiler warning 을 완화한다.

type File = String; // type alias 를 만든다. 컴파일러 는 구분하지 않지만 코드 단에서 구분하기 위함.

fn open(f: &mut File) -> bool {
    true // 항상 성공한다고 가정한다.
}

fn close(f: &mut File) -> bool {
    true // 항상 성공한다고 가정한다.
}

#[allow(dead_code)]
// ! 반환 타입은 이함수가 어떤 값도 반환하지 않는 다고 컴파일러에게 알려주는 역활을 한다.
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!() // 프로그램이 여기 까지 오게 되면 강제로 종료하는 매크로
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(&mut f1, vec![]);
    close(&mut f1);
}
