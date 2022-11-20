#![allow(unused_variables)] // 경고를 내지 않도록 한다.

use std::vec;

// println! 으로 File을 출력할 수 있도록 한다. std::fmt::Debug 트레이트는 매크로 내에서
// {:?} 과 연계하여 File을 출력 가능한 문자열로 바꾼다.
#[derive(Debug)]

struct File {
    name: String,
    data: Vec<u8>, // Vec<u8> 을 사용하면 동적크기 조정과 같은 몇 가지 유용한 편의 기능에 접근할 수 있으므로 파일에 쓰기 작업을 시뮬레이션할 수 있다.!
}

fn open(f: &mut File) -> bool {
    // 3
    true
}

fn close(f: &mut File) -> bool {
    // 4
    true
}

// 읽은 바이트의 수를 반환한다.
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length); // 7
    save_to.append(&mut tmp); // 8
    read_length
}

fn mock_file_read() {
    let mut f2 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];
    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
}

fn main() {
    let f1 = File {
        name: String::from("f1.txt"), // String::from 은 슬라이스인 문자열 리터럴에서 소유한 문자열을 생성한다.
        data: Vec::new(),             // 여기서 vec! 매크로가 빈 파일을 시뮬레이트한다.
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}
