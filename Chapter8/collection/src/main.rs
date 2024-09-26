fn main() {
    let inmutable_v: Vec<i32> = Vec::new();

    let mut mutable_v = Vec::new();

    mutable_v.push(5);
    mutable_v.push(6);
    mutable_v.push(7);
    mutable_v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;


    println!("Hello, world!");
}
