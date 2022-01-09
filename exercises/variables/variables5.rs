// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // *.to_owned() 함수를 사용하여 해당 변수에 있는 값을 가져온다.
    println!("Number plus two is : {}", number.to_owned() + "Two");
}