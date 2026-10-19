use std::io::Write;
fn main() {
let data1 = "Lager, Stout, Non-Alcoholic";
let data2 = "Desperados, Turbo King, Amstel Malta";
let data3 = "Goldberg, Williams, Malta Gold";
let data4 = "Gulder, , Fayrouz";
let data5 = "Heineken";
let data6 = "Star";

let  mut file = std::fs::File::create("Nigerian_Breweries.csv").expect("Create failed");
file.write_all(format!("{}\n", data1).as_bytes())
.expect("Write failed");
file.write_all(format!("{}\n", data2).as_bytes())
.expect("Write failed");
file.write_all(format!("{}\n", data3).as_bytes())
.expect("Write failed");
file.write_all(format!("{}\n", data4).as_bytes())
.expect("Write failed");
file.write_all(format!("{}\n", data5).as_bytes())
.expect("Write failed");
file.write_all(format!("{}\n", data6).as_bytes())
.expect("Write failed");
  
}
