extern crate clap;//导入依赖库

use clap::{Arg, App};//指定使用clap的哪个模块

//将字符串组装成数组打印输出
fn main() {
    //arg::with_name可以使用更简洁的 args_from_usage代替
    let matches = App::new("substrate_command")
      .version("1.0.0")
      .author("jbw")
      .about("")
      .arg(Arg::with_name("print")//切割字符串
            .short("p")
            .help("File to print.")
            .empty_values(false)
      )
            .arg(Arg::with_name("length")//返回字符串长度
            .short("l")
            .help("the string length")
            .empty_values(false)//这个是必须的
    )
    //   .subcommand(subcmd) //创建多层级命令行
      .get_matches();
 
     if let Some(data) = matches.value_of("print") {
        let mut str = data.split("1");
        let mut s1 = "(".to_string();
        let  dot = " , ";
        for ele in str { // iterator <--
            let s2 = ele.to_string();
            s1 += &s2;//借用
            s1 += dot;
        }
        s1 += ")";

        let newObj = s1;//所有权转移  <--

        // println!("print the combin array : {}", newObj);

        Cus_print::cus_print("print the combin array", &newObj);//自定义输出trait  <--
    } 


    if let Some(data) = matches.value_of("length"){
        let length = data.len();

        println!("the string length is : {}",length);
    }


}

trait Custom_format_print {
   fn cus_print(va1: &str, va2 : &str);
}

struct Cus_print {
}

impl Custom_format_print for Cus_print {

    fn cus_print(desc: &str, data : &str){
        println!("CUS- {} {}:",desc,data);
    }

}