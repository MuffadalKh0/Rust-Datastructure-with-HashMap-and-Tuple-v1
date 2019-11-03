use std::collections::HashMap;
#[macro_use] extern crate text_io;
#[macro_use] extern crate prettytable;
use prettytable::Table;

fn main() {
    // Creating HashMap
    let mut students = HashMap::new();
    // Creating Table
    let mut table = Table::new();
    // Creating Heading Row
    table.add_row(row!["ID", "NAME", "EMAIL", "AGE"]);
    // Loop Condition
    let mut run = true;
    // Auto Iterating ID
    let mut sid = 1;
    
    while run {
        println!("Enter Name");
        let name: String = read!();

        println!("Enter Email");
        let email: String = read!();

        println!("Enter Age");
        let age: i32 = read!();

        //Inserting Value Into Hashmap
        students.insert(sid, (name, email, age));

        // Adding HashMap Value to Table 
        for value in students.get(&sid){
            table.add_row(row![sid, value.0, value.1, value.2]);
        };
        
        table.printstd();

        println!("Continue Adding?");
        println!("1)Yes\n0)No");
        let continue_: i32 = read!();
        if continue_ == 0 {
            run = false   
        }
        sid+=1;
    }
}