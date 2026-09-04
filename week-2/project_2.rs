fn main(){
	//laptop companies
	let toshiba: f64 = 2.0;
	let mac: f64 = 1.0;
	let hp: f64 = 3.0;
	let dell: f64 = 3.0;
	let acer: f64 = 1.0;

//laptop totals
let p_toshiba = toshiba * 450000.0;
let p_mac = mac * 1500000.0;
let p_hp = hp * 750000.0;
let p_dell = dell * 2850000.0;
let p_acer = acer * 250000.0;
 
 //average
 let total = toshiba + mac + hp + dell + acer;
 let p_total = p_toshiba+p_mac+p_hp+p_dell+p_acer;
 let average = p_total/total;
 println!("Your average is {}",average );

}