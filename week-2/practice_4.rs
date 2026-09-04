fn main(){
	let p=1000;
	let r=5;
	let t=4;

	//simple interest
	let a= p*(1+(r/100))*t;
	println!("Amount is {}",a );
	let si = a - p;
	println!("Simple Interest is{}",si );

}