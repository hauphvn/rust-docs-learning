// fn five(x: i32) -> bool {
//    if x > 5 {
//        return true;
//    }
//     return false;
// }


fn main() {
    // let five = five(5);
    // let condition  = true;
    // let x = if condition {5} else {"6"};
    // println!("Hello, world! {x}");
// let mut x:i32 = 0;
//     loop {
//        x += 2;
//         if x > 10 {
//             break x += 100;
//         }
//         println!("x: {x}");
//     }
//     println!("final: {x}")
//
//     let mut count  = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 9;
//         loop {
//             remaining += 1;
//             if remaining > 10 {
//                 break;
//             }else{
//                 println!("remaining: {remaining}");
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             count += 1;
//         }
//     }
//     println!("final count: {count}");
//     let a = [1,2,3,4,5];
    // for element in a {
    //     print!("{element}");
    // }

    // for number in (1..4).rev() {
    //     print!("{number}");
    // }

    // let a:[i32; 1] = [5];
    // let mut sum = 0;
    // for x in a {
    //    println!("{x}")
    // }
    // println!("{sum}");

    // let s1 = String::from("hauphvn");
    // let _s2 = s1;
    // println!("s1: {_s2}");

    // let x = String::from("hello");
    // let mut y = x;
    // y.push_str(",world");
    // println!("{y}")
    // let s = String::from("hauphvn");
    // takes_ownership(s);
    // println!("after: {s}");

  //   let x = if true {1} else {"hauphvn"};
  // assert_eq!(x+1, 2)

    // let s = String::from("hello");
    // let s2;
    // let b = false;
    // if b {
    //     s2 = s;
    // }
    // println!("{s}")

    // let mut s1 = String::from("hauphvn");
    // let s2 = change(&mut s1);
    // println!("s1 {s1}");
    // println!("s2 {s2}");
    // print(&s1)

    // let mut n = 1;
    // incr(&mut n);
    // println!("{n}")

    // let mut s = String::from("hauphvn");
    // let s2 = &s;
    // println!("{s2}");
    // let s3 = &mut s;
    // s3.push_str(" changed");
    // println!("{s3}");

    // let s = dangle();

   //  let mut s = String::from("hau phvn");
   // // //  let index =  first_word(&s);
   // // // println!("{index}");
   // //  let hau: &str = &s[0..3];
   // //  let all: &str = &s[..];
   // //  println!("{all}")
   //
   //  let first: &str = first_word(&s);
   //  println!("{first}");
   //  s.clear();
   //  println!("2: {first}")

    // let arr = [1,2,3,4,5];
    // let arr2 = &arr[0..3];
    // assert_eq!(arr2, &arr[1..3]);

    // let mut s = String::from("hello");
    // for &item in s.as_bytes().iter() {
    //     if item == b'l'{
    //         s.push_str(" world");
    //     }
    // }
    // println!("{s}")

    println!("&String = {} &str={}", std::mem::size_of::<&String>(), std::mem::size_of::<&str>())

}

fn takes_ownership(some_string: String) {
    println!("takes ownership: {some_string}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String){
    s.push_str(", changed");
}
fn print(s: &String) {
    println!("{s}")
}

fn incr(s: &mut i32){
    *s += 1
}

fn dangle() -> String {
    let s = String::from("hauphvn");
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
           return &s[0..i];
        }
    }
    &s[..]
}