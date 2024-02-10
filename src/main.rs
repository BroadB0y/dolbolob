use std::io;

fn main() -> io::Result<()> {
    let mut input: String = String::new();
    println!("Введите свой возраст: ");
    io::stdin().read_line(&mut input)?;
    match input.trim() {
        "1488" => println!("Запускаем вентиля, братва"),
        "228" => println!("Мы ещё не вошли в братву"),
        _ => println!("Это не мой день рождения!"),
    };
    Ok(())
}
