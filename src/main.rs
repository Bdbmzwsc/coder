fn main() {
    let mut str = String::new();
    println!("input mode,1 is encryption,0 is decryption");
    std::io::stdin()
        .read_line(&mut str)
        .expect("Error at read value");
    if str == "1\r\n".to_string() {
        std::io::stdin()
            .read_line(&mut str)
            .expect("Error at read value two");
        encryption(str);
    }else if str == "0\r\n".to_string() {
        std::io::stdin()
            .read_line(&mut str)
            .expect("Error at read value two");
        decryption(str);
    }
}
fn encryption(str: String) {
    let mut binary_num: Vec<usize> = Vec::new();
    for e in str[3..str.len() - 2].chars() {
        let mut a: i32 = e as i32;
        let mut c = 0;
        print!("{a}");
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
fn decryption(str: String) {
    let mut i=str.len()-5;
    let mut sum=0;
    for c in str[3..str.len()-2].chars() {
        i-=1;
        if c=='*'{
            let mut t=i;
            let mut sum1=1;
            while t>0{
                sum1=sum1*2;
                t-=1;
            }
            sum+=sum1;
        }
        
    }
    let c: char=char::from_u32(sum).unwrap();
    println!("{c}");
    
}
