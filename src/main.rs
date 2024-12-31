
/******************************************
 * This Program is used to convert physical 
 * quantities form system -> another system 
 * ex: (SI -> British) or (British -> SI)
 * 
 * 
 * Developer name: Mahmoud Ali
 * Date: 31.12.2024
 * Loc: Graz, Austria
 * Version.N : ver 1.0.0
********************************************/

// Import some modules from input output
use std::io::{self, Write};

// activate automatic Debug features, allow cloning and copy
#[derive(Debug,Clone,Copy)]
// Length avaliable units 
enum LenghtUnit{
    Meters,
    Kilometers,
    Miles,
    Feet,
}


#[derive(Debug,Clone,Copy)]
// Weight avaliable units 
enum WeightUnit{

    KiloGrams,
    Pounds,
    Grams,
    Ounces,

}

#[derive(Debug,Clone,Copy)]
// Temperature avaliable units 
enum TemperatureUnit{

    Celsius,
    Fahrenheit,
    Kelvin,

}
//Declare a struct to handle all the conersion process
struct UnitConverter;

// Implement function bounded by the struct
impl UnitConverter{
    // Print the initial menu
    pub fn show_menu()
    {
        println!("Unit Converter CLI");
        println!("1. Convert Length");
        println!("2. Convert Weight");
        println!("3. Convert Temperature");
        println!("4. Exit");
        println!("Choode an option: ");
        // Clarify the output buffer, handle the Std/consol output and check if it is OK or not  
        io::stdout().flush().unwrap();
    }

    // read the input from user to determine the process required
    pub fn handle_input()
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => UnitConverter::convert_length(),
            "2" => UnitConverter::convert_weight(),
            "3" => UnitConverter::convert_temp(),
            "4" => {
                println!("Shutting down the Program!");
                std::process::exit(0); // Exit the program with status code 0
            }
            _ => println!("Invalid option, please try again."),
        }
    }
    // Manage the length physical conversion
    pub fn convert_length()
    {
        println!("\n----------------------Start Length Conversion ----------------------\n");
        println!(" Please Enter  value to convert: ");
        io::stdout().flush().unwrap();
        let mut value_input = String::new();
        io::stdin().read_line(&mut value_input).unwrap();
        let value: f64 = match value_input.trim().parse()
        {
            Ok(v) => v,
            Err(_) =>{
                println!("Invalid value");
                return;
            }
        };

        println!("Choose the input unit:");
        println!("1. Meters");
        println!("2. Kilometers");
        println!("3. Miles");
        println!("4. Feet");
        println!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut input_unit = String::new();
        io::stdin().read_line(&mut input_unit).unwrap();
        let from_unit = match input_unit.trim()
        {
            "1" => LenghtUnit::Meters,
            "2" => LenghtUnit::Kilometers,
            "3" => LenghtUnit::Miles,
            "4" => LenghtUnit::Feet,
            _   => {
                   println!("Invalid choice.");
                   return;
            } 

        };


        println!("Choose the output unit:");
        println!("1. Meters");
        println!("2. Kilometers");
        println!("3. Miles");
        println!("4. Feet");
        println!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut output_unit = String::new();
        io::stdin().read_line(&mut output_unit).unwrap();
        let to_unit = match output_unit.trim()
        {
            "1" => LenghtUnit::Meters,
            "2" => LenghtUnit::Kilometers,
            "3" => LenghtUnit::Miles,
            "4" => LenghtUnit::Feet,
            _   => {
                   println!("Invalid choice.");
                   return;
            } 

        };


        let result = convert_length(value, from_unit, to_unit);
        println!("Result: {:.2} {:?} is {:.2} {:?}",value,from_unit,result,to_unit);

    }

    // Manage the weight conversion
    pub fn convert_weight()
    {
        // 1. Print the identification message.
        println!("\n----------------------Start Weight Conversion ----------------------\n");
        println!("Please Enter amount to convert: ");
        io::stdout().flush().unwrap();

        // 2. Print the avaliable list ->  for input to convert.
        let mut value_input = String::new();
        io::stdin().read_line(&mut value_input).unwrap();
        let value : f64 = match value_input.trim().parse()
        {
            Ok(v) => v,
            Err(_) => {
                print!("Invalid value Inserted");
                return;
            }
        };
         
        // 3. Create the variable and match it to know what is the input.
        println!("Please Choose the input unit");
        println!("1. Kilograms");
        println!("2. Pounds");
        println!("3. Grams");
        println!("4. Ounce");
        println!("Print your choice: ");
        io::stdout().flush().unwrap();
        let mut input_data = String::new();
        io::stdin().read_line(&mut input_data).unwrap();
        let from_unit = match input_data.trim()
        {
            "1" => WeightUnit::KiloGrams,
            "2" => WeightUnit::Pounds,
            "3" => WeightUnit::Grams,
            "4" => WeightUnit::Ounces,
            _ => {
                println!("Invalid Input");
                return;
            }
        };
        // 4. Print the avaliable list -> for output.
        // 5. Create input to get the desired output unit. 
        println!("Please Choose the Output unit");
        println!("1. Kilograms");
        println!("2. Pounds");
        println!("3. Grams");
        println!("4. Ounce");
        println!("Print your choice: ");
        io::stdout().flush().unwrap();
        let mut output_data = String::new();
        io::stdin().read_line(&mut output_data).unwrap();
        let to_unit = match output_data.trim()
        {
            "1" => WeightUnit::KiloGrams,
            "2" => WeightUnit::Pounds,
            "3" => WeightUnit::Grams,
            "4" => WeightUnit::Ounces,
            _ => {
                println!("Invalid Input");
                return;
            }
        };
        // 6. Call the funtion for Conversion.
        let result : f64 = convert_weight(value, from_unit, to_unit);
        println!("Result: {:.2} {:?} is {:.2} {:?}",value,from_unit,result,to_unit);

    }

    // Manage the temp conversion
    pub fn convert_temp()
    {
        println!("\n---------------------- Start Temperature Conversion ----------------------\n");
        println!("Enter the amount to convert: ");
        io::stdout().flush().unwrap();
        let mut value_input = String::new();
        io::stdin().read_line(&mut value_input).unwrap();
        let value : f64 = match value_input.trim().parse()
        {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid input value");
                return;
            }
        };

        // 3. Create the variable and match it to know what is the input.
        println!("Please Choose the input unit");
        println!("1. Celsius");
        println!("2. Fahrenheit");
        println!("3. Kelvin");
        println!("Print your choice: ");
        io::stdout().flush().unwrap();
        let mut input_data = String::new();
        io::stdin().read_line(&mut input_data).unwrap();
        let from_unit = match input_data.trim()
        {
            "1" => TemperatureUnit::Celsius,
            "2" => TemperatureUnit::Fahrenheit,
            "3" => TemperatureUnit::Kelvin,
            _ => {
                println!("Invalid Input");
                return;
            }
        };
        // 4. Print the avaliable list -> for output.
        // 5. Create input to get the desired output unit. 
        println!("Please Choose the Output unit");
        println!("1. Celsius");
        println!("2. Fahrenheit");
        println!("3. Kelvin");
        println!("Print your choice: ");
        io::stdout().flush().unwrap();
        let mut output_data = String::new();
        io::stdin().read_line(&mut output_data).unwrap();
        let to_unit = match output_data.trim()
        {
            "1" => TemperatureUnit::Celsius,
            "2" => TemperatureUnit::Fahrenheit,
            "3" => TemperatureUnit::Kelvin,
            _ => {
                println!("Invalid Input");
                return;
            }
        };
        // 6. Call the funtion for Conversion.
        let result : f64 = convert_temperature(value, from_unit, to_unit);
        println!("Result: {:.2} {:?} is {:.2} {:?}",value,from_unit,result,to_unit);  
        
    }
} // End of the struct implementaion

// The function handles length logic conversion
fn convert_length(value: f64, from: LenghtUnit, to: LenghtUnit) -> f64 {
    match (from, to) {
        // Same unit, no conversion needed
        (LenghtUnit::Meters, LenghtUnit::Meters) => value,
        (LenghtUnit::Kilometers, LenghtUnit::Kilometers) => value,
        (LenghtUnit::Miles, LenghtUnit::Miles) => value,
        (LenghtUnit::Feet, LenghtUnit::Feet) => value,

        // Meters to other units
        (LenghtUnit::Meters, LenghtUnit::Kilometers) => value / 1000.0,
        (LenghtUnit::Meters, LenghtUnit::Miles) => value * 0.000621371,
        (LenghtUnit::Meters, LenghtUnit::Feet) => value * 3.28084,

        // Kilometers to other units
        (LenghtUnit::Kilometers, LenghtUnit::Meters) => value * 1000.0,
        (LenghtUnit::Kilometers, LenghtUnit::Miles) => value * 0.621371,
        (LenghtUnit::Kilometers, LenghtUnit::Feet) => value * 3280.84,

        // Miles to other units
        (LenghtUnit::Miles, LenghtUnit::Meters) => value / 0.000621371,
        (LenghtUnit::Miles, LenghtUnit::Kilometers) => value / 0.621371,
        (LenghtUnit::Miles, LenghtUnit::Feet) => value * 5280.0,

        // Feet to other units
        (LenghtUnit::Feet, LenghtUnit::Meters) => value / 3.28084,
        (LenghtUnit::Feet, LenghtUnit::Kilometers) => value / 3280.84,
        (LenghtUnit::Feet, LenghtUnit::Miles) => value / 5280.0,
    }
}

// The function handles weight logic conversion
fn convert_weight(value: f64 , from: WeightUnit, to: WeightUnit) -> f64 {

    match(from, to){
        
        // Kilo to other
        (WeightUnit::KiloGrams,WeightUnit::Pounds) => value * 2.20462,
        (WeightUnit::KiloGrams,WeightUnit::Grams) => value * 1000.0,
        (WeightUnit::KiloGrams,WeightUnit::Ounces) => value *35.274,

        // Gram to other
        (WeightUnit::Grams,WeightUnit::KiloGrams) => value / 1000.0,
        (WeightUnit::Grams,WeightUnit::Pounds) => value / 453.592,
        (WeightUnit::Grams,WeightUnit::Ounces) => value / 28.3495,

        // Pounds to other
        (WeightUnit::Pounds, WeightUnit::KiloGrams) => value / 2.20462,
        (WeightUnit::Pounds, WeightUnit::Grams) => value * 453.592,
        (WeightUnit::Pounds, WeightUnit::Ounces) => value * 16.0,

        // Ounces to other
        (WeightUnit::Ounces,WeightUnit::KiloGrams) => value / 35.274,
        (WeightUnit::Ounces,WeightUnit::Pounds) => value / 16.0,
        (WeightUnit::Ounces,WeightUnit::Grams) => value * 28.3495,

        // No Conversion needed 
        (_ , _) => value,
 
    }
}

// The function handles Temp logic conversion
fn convert_temperature(value:f64, from:TemperatureUnit, to:TemperatureUnit ) -> f64{
    match(from, to){
        (TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit) => value * 1.8 + 32.0,
        (TemperatureUnit::Celsius, TemperatureUnit::Kelvin) => value + 273.15,

        (TemperatureUnit::Fahrenheit, TemperatureUnit::Fahrenheit) => (value - 32.0) / 1.8 ,
        (TemperatureUnit::Fahrenheit, TemperatureUnit::Kelvin) => (value - 32.0) / 1.8 + 273.15,
        
        (TemperatureUnit::Kelvin, TemperatureUnit::Celsius) => value - 273.15,
        (TemperatureUnit::Kelvin, TemperatureUnit::Fahrenheit) => (value - 273.15)*1.8 + 32.0,        
        (_,_) => value,
    }
}


// The Entry Point of the function
fn main() {
    loop{
        // Call the menu function implemented in the struct
        UnitConverter::show_menu();
        // Call handle input 
        UnitConverter::handle_input();
    }
}
