#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, Map};

// Struktur data untuk status loyalitas pelanggan
#[contracttype]
#[derive(Clone, Debug)]
pub struct CustomerStatus {
    pub service_count: u32,
    pub has_free_service: bool,
}

const LOYALTY_DATA: Symbol = symbol_short!("LOYALTY");

#[contract]
pub struct ProCleanLoyaltyContract;

#[contractimpl]
impl ProCleanLoyaltyContract {
    // Fungsi untuk menambah riwayat servis pelanggan
    pub fn add_service(env: Env, customer_id: Symbol) -> CustomerStatus {
        let mut loyalty_map: Map<Symbol, CustomerStatus> = env.storage().instance()
            .get(&LOYALTY_DATA)
            .unwrap_or(Map::new(&env));

        let mut status = loyalty_map.get(customer_id.clone()).unwrap_or(CustomerStatus {
            service_count: 0,
            has_free_service: false,
        });

        // Logika Bisnis: Tambah poin. Jika sudah 5, dapat reward.
        status.service_count += 1;
        if status.service_count >= 5 {
            status.has_free_service = true;
        }

        loyalty_map.set(customer_id, status.clone());
        env.storage().instance().set(&LOYALTY_DATA, &loyalty_map);
        
        status
    }

    // Fungsi untuk menggunakan reward (reset poin ke 0)
    pub fn claim_reward(env: Env, customer_id: Symbol) -> bool {
        let mut loyalty_map: Map<Symbol, CustomerStatus> = env.storage().instance()
            .get(&LOYALTY_DATA)
            .unwrap_or(Map::new(&env));

        if let Some(mut status) = loyalty_map.get(customer_id.clone()) {
            if status.has_free_service {
                status.service_count = 0;
                status.has_free_service = false;
                loyalty_map.set(customer_id, status);
                env.storage().instance().set(&LOYALTY_DATA, &loyalty_map);
                return true;
            }
        }
        false
    }
}