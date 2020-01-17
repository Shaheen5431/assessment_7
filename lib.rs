pub mod ogdcl {
    pub fn zamzama_gas_field(){
        let field_manager = "Omar Burki".to_string();
        let gas_prod:u32 = 140;
        let budget:u32  = 90;
        
        println!("");
        println!("Field Manager of Zamzama Gas Field = '{}'",field_manager);
        println!("Profit from ZAMZAMA GAS FIELD = Production ({}) - Budget ({}) = {}",gas_prod,budget,gas_prod-budget);
    }
}