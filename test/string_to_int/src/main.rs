fn main() {

	let mut v: Vec<String> = vec![];
	v.push("1,25454,3.6".to_string());
	
	let mut str = v[0].clone();
	
	//str enthält jetzt den String
	
	//testen, ob '.' enthalten ist -> float
	let f = match str.find('.'){
		Some(_) => true,
		None => false
	};
	println!("{}",f);
	
	//wandel str in Integer
	let x = match "4".parse::<i32>(){
		Ok(x) => x,
		Err(e) => panic!("Fehler: {}",e)
	};
    println!("{}", x);
	
	let chars: Vec<char> = str.chars().collect();
	
	//nimm ein Zeichen aus Chars
	let mut tmp = String::new();
	tmp.push(chars[0]);
	
	//wandele tmp in einen Integer
	let y = tmp.parse::<i32>().unwrap();
	
	
    println!("vec {:?}",v);
    
    println!("str {:?}",chars);
    println!("{}",y);
    
    
}
