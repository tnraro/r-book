use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
  println!("숫자를 추측해보자!");
  let secret_number = rand::thread_rng().gen_range(1..=100);
  loop {
    println!("숫자를 입력해보세요.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("줄 읽기 실패");
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        eprintln!("숫자를 입력해주세요!");
        continue;
      }
    };
    println!("당신이 추측한 숫자는: {guess}");
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("너무 작아요!"),
      Ordering::Greater => println!("너무 커요!"),
      Ordering::Equal => {
        println!("맞았어요!");
        break;
      }
    }
  }
}
