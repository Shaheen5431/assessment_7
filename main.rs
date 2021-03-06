mod mari_petroleum {
    pub mod field_areas {
        pub fn zarghoon_gas_field(fm:String,gp:u32,bgt:u32){
            println!("");

            println!("Field Manager of Zarghoon Gas (ZG) Field is {}",fm);
            println!("Profit from ZARGHOON GAS FIELD = Production ({}) - Budget ({}) = {}",gp,bgt,gp-bgt);
        }        
    }    
}
mod lib;
use shaheen;
use std::io;
fn main() {
    println!("Please Enter the Name of the Field Manager");

    let mut field_manager = String::new();
    io::stdin().read_line(&mut field_manager).expect("SOME NAME");
    let gas_prod:u32 = 110;
    let budget:u32  = 80;
    mari_petroleum::field_areas::zarghoon_gas_field(field_manager,gas_prod,budget);
    lib::ogdcl::zamzama_gas_field();
    shaheen::pak_petroleum::total_operational_portfolios::exploration_portfolios();
}