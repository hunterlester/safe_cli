use std::io;

pub fn read_line(var: &mut String) -> String {
    // TODO: Properly handle sensitive input
    io::stdin().read_line(var).expect("Failed to read line");
    var.trim().to_string()
}
