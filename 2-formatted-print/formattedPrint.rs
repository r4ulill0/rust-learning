
fn main() {

    println!("Normal {{}} formatting {}", 3);
    println!("Positional formatting {0}", 3);
    println!("Named formatting {{name}} {name}", name=3);
    println!("Special binary formatting {{:b}} {:b}", 3);
    println!("Special alignment {{number:>width$}} {number:>width$} with number={number} and width={width}", number=2, width=3);
    println!("Special padding with zeroes {{number:0>width$}} {number:0>width$} with number={number} and width={width}", number=2, width=3);
    println!("How many X's do you want? Maybe this much? {:X>number$}", number=6);
    println!("\n ######################\n\nALL OF THIS IS EXPLAINED IN:\n\n std::fmt \n\n THERE IS WERE THIS MACRO BEHAVIOUR IS DEFINED");

}
