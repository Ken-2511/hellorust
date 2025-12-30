use std::io;

fn main() {
    println!("你好！请输入你的名字：");

	let mut name = String::new();
	
	io::stdin()
		.read_line(&mut name)
		.expect("读取输入失败");
	
	println!("你好，{}！很高兴见到你！", name.trim());
}
