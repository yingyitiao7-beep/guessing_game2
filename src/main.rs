use rand::RngExt;//如今rand库已经不再使用rand::Rng，而是使用rand::RngExt来提供随机数生成器的扩展方法
use std::io;//std是rust的库，io = input output 输入输出
use std::cmp::Ordering;//cmp = copmare比较函数,ordering是一个枚举类型，包含Less, Greater, Equal不用每次都写less等

fn main(){
    println!("猜数字");

    let secret_number: u32 = rand::rng().random_range(1..100);//单个冒号用来标注变量或者值的类型，而两个冒号用于路径的访问：库，模块，内部函数，方法等
    //让secret_number为32位无符号整型，然后执行rand库也就是随机数，之后在调用了random_range把数字范围执行到（1到100）之间
    //println!("the secret number is: {}", secret_number);

    loop {//开始loop 循环
        println!("请输入你猜的数字: ");
        let mut guess:String  = String::new();
        //给我键盘输入的数字开辟一个存放空间,mut 则让guess成为可变变量

        io::stdin()//stdin相当于开始调用键盘输入
            .read_line(&mut guess)//read_line就是开始读取，抓获键盘输入赋给guess
            .expect("读取失败");//如果程序失败就输出这个
        //以上我们的guess是一个字符串类型的变量，而我们需要的是一个数字类型的变量，所以我们需要将字符串类型的guess转换为数字类型的guess
        let guess: u32 = guess.trim().parse().expect("请输入一个数字");
                                //让 gusess为32位无符号整型
                                //trim()是去掉字符串前后的空格，因为read_line自带多余字符，会导致格式转换失败 键盘输入会自带\n换行符，trim()就是去掉这个换行符
                                //parse()是将字符串转换为数字类型
        println!("你猜的数字是: {}", guess);
        
        match guess.cmp(&secret_number) {
        //开始match语句，强制穷尽匹配原则，之后调用比较函数,&是取引用 将值调用到这里，但是所有权还是在本身代码中
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("你赢了");//以上就是结果的输出
                break;//只作用于跳出外面loop的循环，在此期间match本身不需要跳出，match本身自己就会结束
            }
        }
            
    }
}
