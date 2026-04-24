#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map};

#[contract]
pub struct EventTickets;

#[contractimpl]
impl EventTickets {
    /// Initialize the contract
    pub fn init(env: Env) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
    }

    /// Create a new event with specified total tickets and price
    pub fn create_event(env: Env, event_id: u64, total_tickets: u64, price: u64) {
        let mut events: Map<u64, (u64, u64, u64)> = env
            .storage()
            .instance()
            .get(&"events")
            .unwrap_or(Map::new(&env));

        if events.contains_key(event_id) {
            panic!("Event already exists");
        }

        events.set(event_id, (total_tickets, 0, price));
        env.storage().instance().set(&"events", &events);
    }

    /// Buy a ticket for an event. Decrements available tickets and records buyer.
    pub fn buy_ticket(env: Env, event_id: u64, buyer: Address) -> bool {
        buyer.require_auth();

        let mut events: Map<u64, (u64, u64, u64)> = env
            .storage()
            .instance()
            .get(&"events")
            .unwrap_or(Map::new(&env));

        let (total, sold, price) = events.get(event_id).unwrap_or((0, 0, 0));

        if sold >= total {
            panic!("No tickets available");
        }

        let mut ownership: Map<(u64, Address), bool> = env
            .storage()
            .instance()
            .get(&"ownership")
            .unwrap_or(Map::new(&env));

        let key = (event_id, buyer.clone());
        if ownership.get(key.clone()).unwrap_or(false) {
            panic!("Buyer already owns a ticket");
        }

        ownership.set(key, true);
        env.storage().instance().set(&"ownership", &ownership);

        events.set(event_id, (total, sold + 1, price));
        env.storage().instance().set(&"events", &events);

        true
    }

    /// Get the remaining ticket count for an event
    pub fn get_ticket_count(env: Env, event_id: u64) -> u64 {
        let events: Map<u64, (u64, u64, u64)> = env
            .storage()
            .instance()
            .get(&"events")
            .unwrap_or(Map::new(&env));

        let (total, sold, _) = events.get(event_id).unwrap_or((0, 0, 0));
        total - sold
    }

    /// Check if an address owns a ticket for an event
    pub fn get_ticket_owner(env: Env, event_id: u64, buyer: Address) -> bool {
        let ownership: Map<(u64, Address), bool> = env
            .storage()
            .instance()
            .get(&"ownership")
            .unwrap_or(Map::new(&env));

        ownership.get((event_id, buyer)).unwrap_or(false)
    }
}
