use std::io;

fn celsia_converter(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
fn main() {
    println!("Введите температуру в градусах Цельсия:");
    let mut celsius = String::new();
    io::stdin().read_line(&mut celsius).expect("Ошибка ввода");
    let celsius: f64 = celsius.trim().parse().expect("Вставьте число");
    let fahrenheit = celsia_converter(celsius);
    println!("Температура в градусах Фаренгейта: {:.2}", fahrenheit);
}