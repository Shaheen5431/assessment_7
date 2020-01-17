pub mod pak_petroleum {
    pub mod total_operational_portfolios {
        pub fn exploration_portfolios() {
            let blocks_in_sindh = 10;
            let blocks_in_kpk = 5;
            let blocks_in_balochistan = 12;
            let blocks_in_punjab = 2;
            println!("");
	    println!("Exploration Portfolios of Pak Petroleum in Sindh = {}",blocks_in_sindh);
            println!("Exploration Portfolios of Pak Petroleum in KPK = {}",blocks_in_kpk);
            println!("Exploration Portfolios of Pak Petroleum in Balochistan = {}",blocks_in_balochistan);
            println!("Exploration Portfolios of Pak Petroleum in Punjab = {}",blocks_in_punjab);
            println!("Total Exploration Portfolios of Pak Petroleum = {}+{}+{}+{} = {}",blocks_in_sindh,blocks_in_kpk,blocks_in_balochistan,blocks_in_punjab, blocks_in_sindh+blocks_in_kpk+blocks_in_balochistan+blocks_in_punjab);
        }
    }
}
 
