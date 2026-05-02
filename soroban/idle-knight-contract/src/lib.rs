#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, String, Symbol};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Creator(Address),
    Content(u64),
    Sub(Address, Address),
    NextContentId,
}

#[derive(Clone)]
#[contracttype]
pub struct CreatorProfile {
    pub wallet: Address,
    pub handle: String,
    pub bio: String,
    pub content_ref: String,
}

#[derive(Clone)]
#[contracttype]
pub struct ContentEntry {
    pub id: u64,
    pub creator: Address,
    pub content_ref: String,
    pub premium: bool,
}

#[contract]
pub struct IdleKnight;

#[contractimpl]
impl IdleKnight {
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) { panic!("already initialized"); }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::NextContentId, &1u64);
    }

    pub fn upsert_creator(env: Env, creator: Address, handle: String, bio: String, content_ref: String) {
        creator.require_auth();
        let profile = CreatorProfile { wallet: creator.clone(), handle, bio, content_ref };
        env.storage().persistent().set(&DataKey::Creator(creator), &profile);
    }

    pub fn publish_content(env: Env, creator: Address, content_ref: String, premium: bool) -> u64 {
        creator.require_auth();
        let mut next: u64 = env.storage().instance().get(&DataKey::NextContentId).unwrap_or(1);
        let id = next;
        next += 1;
        let entry = ContentEntry { id, creator, content_ref, premium };
        env.storage().persistent().set(&DataKey::Content(id), &entry);
        env.storage().instance().set(&DataKey::NextContentId, &next);
        env.events().publish((Symbol::new(&env, "content_published"), id), entry.premium);
        id
    }

    pub fn subscribe(env: Env, subscriber: Address, creator: Address, token_id: Address, amount: i128, duration_secs: u64, now: u64) {
        subscriber.require_auth();
        if amount <= 0 || duration_secs == 0 { panic!("bad params"); }
        let token = token::Client::new(&env, &token_id);
        token.transfer(&subscriber, &creator, &amount);
        let expiry = now.saturating_add(duration_secs);
        env.storage().persistent().set(&DataKey::Sub(subscriber.clone(), creator.clone()), &expiry);
        env.events().publish((Symbol::new(&env, "subscription_created"), subscriber, creator), expiry);
        env.events().publish(Symbol::new(&env, "payment_processed"), amount);
    }

    pub fn has_active_subscription(env: Env, subscriber: Address, creator: Address, now: u64) -> bool {
        let expiry: u64 = env.storage().persistent().get(&DataKey::Sub(subscriber, creator)).unwrap_or(0);
        expiry > now
    }

    pub fn can_access_content(env: Env, viewer: Address, content_id: u64, now: u64) -> bool {
        let content: ContentEntry = env.storage().persistent().get(&DataKey::Content(content_id)).unwrap();
        let allowed = !content.premium || viewer == content.creator || Self::has_active_subscription(env.clone(), viewer.clone(), content.creator.clone(), now);
        env.events().publish((Symbol::new(&env, "content_access_checked"), viewer, content_id), allowed);
        allowed
    }
}
