
fn main() {
    let mut variable_variable = 1;
    variable_variable += 1;
    println!("Mira como crece {}", variable_variable);
    {
        let mut variable_variable = variable_variable;
        variable_variable +=1;
        println!("Mira como crece {}", variable_variable);
    }
    println!("Wait what? {}... vaya shadowing", variable_variable);
}
