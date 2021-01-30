fn main() {
    lop();
    ifs();
    println!("bx");
    bx();
    println!("vec");
    vec();
    println!("then");
    then();
    println!("lib");
    lib();
    println!("typing");
    typing();
    data();
    whl();
    lp();
    bk();
    println!("start");
    mcr();
}
fn mcr(){
    println!("{}", file!());
    println!("{}", line!());
    println!("{}", cfg!(unix));
    println!("{}", env!("PATH"));
}
fn bk() {
    /*
    'main: loop {
        println!("main loop start");
        'sub: loop {
            println!("sub loop start");
            break: 'main;
            println!("sub loop end");
        }
        println!("main loop end");
    }*/
    let s = concat!("A","b2");
    println!("{}",s);
    let s = format!("{}-{:?}", s, ("D",5));
    println!("{}",s);

}
fn lp() {
    for count in 0..10 {
        println!("{}", count);
    }
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for ele in &array {
        println!("ele:{}", ele);
    }
}
fn whl() {
    let mut count = 0;
    while count < 10 {
        println!("count:{}", count);
        count += 1;
    }
}
fn lop() {
    let mut count = 0;
    let result = loop {
        println!("count:{}", count);
        count += 1;
        if count == 10 {
            break count;
        }
    };
    println!("{:?}", result);
}
fn ifs() {
    let number = 1;
    if 0 < number {
        println!("0 < number");
    } else if number < 0 {
        println!("number < 0");
    } else {
        println!("0 == number")
    }
    let number = 1;
    let result = if 0 <= number { number } else { -number };
    println!("{:?}", result);
}
fn bx() {
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    print(Box::new(byte_array));
}
fn print(s: Box<[u8]>) {
    println!("{:?}", s);
}
fn vec() {
    let v1 = vec![1, 2, 3, 4, 5];
    println!("{:?}", v1);
    let v2 = vec![0; 5];
    println!("{:?}", v2);
    let v = vec![0; 5];
    for element in &v {
        println!("{}", element)
    }
}
fn then() {
    fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
        let code = result?;
        println!("code:{}", code);
        Ok(100)
    }
    fn func(code: i32) -> Result<i32, String> {
        println!("code:{}", code);
        Ok(100)
    }
    let result: Result<i32, String> = Ok(200);
    let next_result = result.and_then(func);
    println!("{:?}", next_result);
    let result: Result<i32, String> = Err("error".to_string());
    let next_result = result.and_then(func);
    println!("{:?}", next_result);

    let result: Result<i32, String> = Err("error".to_string());
    error_handling(result);
    let result: Result<i32, String> = Ok(10);
    error_handling(result);
}

fn lib() {
    let result: Result<i32, String> = Ok(200);
    match result {
        Ok(code) => println!("code:{}", code),
        Err(err) => println!("Err:{}", err),
    }

    let result2: Result<i32, String> = Ok(400);
    if let Ok(code) = result2 {
        println!("code:{}", code);
    }

    let result3: Result<i32, String> = Ok(600);
    println!("code{}", result3.unwrap_or(-1));
    let result3: Result<i32, String> = Err("error".to_string());
    println!("code{}", result3.unwrap_or(-1));
}

fn typing() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }
    let p = Person {
        name: String::from("takahiro"),
        age: 8,
    };

    println!("{:?}", p);
    #[derive(Debug)]
    enum Event {
        Click,
        KeyDown(u32),
    }
    let event = Event::KeyDown(9);
    let event2 = Event::Click;

    println!("{:?}{:?}", event, event2);
}

fn data() {
    let s1: String = String::from("Hello");
    let s2: &str = &s1;
    let s3: String = s2.to_string();
    println!("{:?}", s3);

    let mut t = (1, "2");
    t.0 = 3;
    t.1 = "5";
    println!("{:?}", t);

    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [4; 3];
    a[1] = b[1];
    a[2] = b[2];

    println!("{:?}", a);
}
