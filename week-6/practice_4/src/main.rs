fn main() {
    let fullname = "Chidubem John Umeh";
    let department =  "Computer Science";
    let uni = "Pan-Atlantic Univesity";

     let mut school = "School of Science and Technology".to_string();

     // pushing the string
     school.push_str("and Technology");
     println!("My name is : {}",fullname );

// length check

println!("The length of my fullname is {}", fullname.len());
println!("Iam a student of {} Department", department );
println!("{}", school );
println!("{}", uni);

}