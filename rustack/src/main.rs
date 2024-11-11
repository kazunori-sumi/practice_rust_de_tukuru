#[derive(Debug, PartialEq, Eq)]

// 'src : ライフタイム指定（ジェネリクスの一種）
// `ソース文字列への参照を持ち、ソース文字列の寿命に依存する
enum Value<'src> {
	Num(i32),
	Op(&'src str),
	Block(Vec<Value<'src>>), // Vec に包むことで Vec 型を介するようにして循環参照を回避
}

enum ValueType {
	Num,
	Op,
	Block,
}

fn main() {
	for line in std::io::stdin().lines() {
			let mut stack = vec![];
			if let Ok(line) = line {
					let words: Vec<_> = line.split(" ").collect();

					for word in words {
							if let Ok(parsed) = word.parse::<i32>() {
									stack.push(parsed);
							} else {
									match word {
											"+" => add(&mut stack),
											"-" => sub(&mut stack),
											"*" => mul(&mut stack),
											"/" => div(&mut stack),
											_ => panic!("{word:?} could not be parsed"),
									}
							}
					}

					println!("stack:{stack:?}");
			}
	}
}

fn add(stack: &mut Vec<i32>) {
	let lhs = stack.pop().unwrap();
	let rhs = stack.pop().unwrap();

	stack.push(lhs + rhs);
}

fn sub(stack: &mut Vec<i32>) {
	let lhs = stack.pop().unwrap();
	let rhs = stack.pop().unwrap();

	stack.push(lhs - rhs);
}

fn mul(stack: &mut Vec<i32>) {
	let lhs = stack.pop().unwrap();
	let rhs = stack.pop().unwrap();

	stack.push(lhs * rhs);
}

fn div(stack: &mut Vec<i32>) {
	let lhs = stack.pop().unwrap();
	let rhs = stack.pop().unwrap();

	stack.push(lhs / rhs);
}
