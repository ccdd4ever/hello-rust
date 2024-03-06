fn main() {
    let s1 = String::from("hello");
    let mut a = 4;
    println!("{}", a);
    double(a);
    println!("{}", a);


    // 所有权从s1移动到f1的参数
    // 然后f1返回值的所有权移动给s2
    let s2 = f1(s1);
    // 注意，println!()不会转移参数s2的所有权
    println!("{}", s2);

    let x = 4;
    f2(x); // 没有移动所有权，而是拷贝一份给f2参数
} // 首先x跳出作用域，
// 然后s2跳出作用域，并释放对应堆内存数据，
// 最后s1跳出作用域，s1没有所有权，所以没有任何其他影响

fn f1(s: String) -> String {
    let ss = String::from("world");
    println!("{},{}", s, ss);
    s // 返回值s的所有权移动到函数外
} // ss跳出作用域

fn f2(i: i32) {
    println!("{}", i);
}

fn double(mut a: i32) -> i32 {
    a = a * 2;
    println!("{}", a);
    a
}