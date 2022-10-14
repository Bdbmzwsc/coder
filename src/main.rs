fn main() {
    let mut str = String::new();
    println!("input chars");
    std::io::stdin()
        .read_line(&mut str)
        .expect("Error at read value");
    let mut binary_num: Vec<usize> = Vec::new();
    for c in str[0..str.len() - 2].chars() {
        let mut a: i32 = c as i32;
        let mut c = 0;

        while c != 1 {
            let b = (a % 2) as usize;
            binary_num.push(b);
            a /= 2;
            c = a;
        }
        binary_num.push(1);
        for i in binary_num.iter().rev() {
            if i == &1 {
                print!("*");
            } else {
                print!("_");
            }
        }
        println!("");
    }
}
