pub fn main() {
    // let x = 100u8;
    // let y: Option<&u8> = Some(&x);
    // let z: Option<u8> = y.copied(); // 将引用转为对应的值，Option的方法

    // println!("{:?}", z);
    // println!("{}", z.unwrap());
    // #[derive(Debug)]
    // struct Point<T> {
    //     x: T,
    //     y: T,
    // }

    // let integer = Point { x: 1, y: 1 };
    // println!("{:#?}", integer);

    // let str1 = "你好".to_string();
    // str1.chars().for_each(|c| println!("{}", c));

    let _ = (0..5)
        // .inspect(|x| println!("before flat_map: {}", x))
        .flat_map(|x| x * 100..x * 110)
        // .inspect(|x| println!("after flat_map: {}", x))
        .enumerate()
        // .inspect(|(i, v)| println!("index:{}, value:{}", i, v))
        .filter(|&(i, x)| (i + x) % 3 == 0)
        // .collect::<Vec<(_, _)>>();
        .for_each(|(i, x)| println!("{}:{}", i, x));
}
