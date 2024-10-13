fn invert(s: String) -> String {
    // Використовуємо chars() для ітерації по символах рядка
    s.chars()
        .map(|c| {
            // Перевіряємо, чи є символ заглавним
            if c.is_uppercase() {
                // Якщо так, то перетворюємо його на малу букву
                c.to_lowercase().to_string()
            } else {
                // Якщо символ малий, перетворюємо його на велику букву
                c.to_uppercase().to_string()
            }
        })
        // Збираємо результати в нову строку
        .collect()
}

fn main() {
    // Перевіряємо, що функція працює правильно
    assert_eq!(invert("Hello".to_string()), "hELLO");
    // Додатковий приклад використання функції
    println!("{}", invert("RustProgramming".to_string())); // Виведе "rUSTpROGRAMMING"
}
