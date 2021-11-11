fn main()
{
    println!("Hello, world!");

// println!マクロ練習：改行
	print!("This is Rust Reasion ");								// print!マクロは改行なし
	println!("with Cargo");											// println!マクロは改行あり

// println!マクロ練習：変数出力
    let x = 10;
    let y = 20;

	println!("x = { } and y = { }", x, y);							// { }の中に後述の変数値を出力

//  println!マクロ練習：数値の出力指定
	let x = 0.12345;
	let y = 987.65;

	println!("x = {:.10}", x);											// 小数点以下指定
	println!("y = {:010}", y);										// 桁数指定(小数点も桁数に含む)
	println!("x = {:E}", x);											// 指数表示
	println!("y = {:e}", y);											// 指数表示

// println!マクロ練習：プレースホルダー
	let one = 1;
	let two = 2;
	let three = 3;
	let four = 4;

	println!("*{3} | { } | { } | *{two} | { } | { } | *{0} | *{four}", one, three, two = two, four = four);

}

