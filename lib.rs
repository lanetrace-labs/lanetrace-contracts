#![no_std]
use soroban_sdk::{contract,contractimpl,contracttype,Address,Env};
#[contracttype]
#[derive(Clone)]
pub enum Key {Owner,Global,Slot(u32)}
#[contract]
pub struct Slots;
fn owner(env:&Env){let a:Address=env.storage().instance().get(&Key::Owner).unwrap();a.require_auth();}
fn add(env:&Env,key:Key,amount:u64)->u64{let n:u64=env.storage().persistent().get(&key).unwrap_or(0);let next=n.checked_add(amount).expect("slot overflow");env.storage().persistent().set(&key,&next);next}
#[contractimpl]
impl Slots {
 pub fn __constructor(env:Env,owner:Address){env.storage().instance().set(&Key::Owner,&owner);}
 pub fn bump_isolated(env:Env,slot:u32,amount:u64)->u64{owner(&env);assert!(slot<16 && amount>0,"invalid input");add(&env,Key::Slot(slot),amount)}
 pub fn bump_shared(env:Env,slot:u32,amount:u64)->u64{owner(&env);assert!(slot<16 && amount>0,"invalid input");add(&env,Key::Global,amount);add(&env,Key::Slot(slot),amount)}
 pub fn read(env:Env,slot:u32)->u64{assert!(slot<16,"invalid slot");env.storage().persistent().get(&Key::Slot(slot)).unwrap_or(0)}
 pub fn global(env:Env)->u64{env.storage().persistent().get(&Key::Global).unwrap_or(0)}
}
#[cfg(test)]
mod tests {use super::*;use soroban_sdk::testutils::Address as _;
fn setup()->(Env,Address){let e=Env::default();e.mock_all_auths();let a=Address::generate(&e);let id=e.register(Slots,(&a,));(e,id)}
#[test]fn isolated_slots_do_not_update_aggregate(){let(e,id)=setup();let c=SlotsClient::new(&e,&id);c.bump_isolated(&1,&5);c.bump_isolated(&2,&8);assert_eq!(c.read(&1),5);assert_eq!(c.read(&2),8);assert_eq!(c.global(),0);}
#[test]fn shared_layout_updates_aggregate(){let(e,id)=setup();let c=SlotsClient::new(&e,&id);c.bump_shared(&1,&5);c.bump_shared(&2,&8);assert_eq!(c.global(),13);assert_eq!(c.read(&1),5);}
#[test]fn reads_have_no_mutation(){let(e,id)=setup();let c=SlotsClient::new(&e,&id);assert_eq!(c.read(&1),0);assert_eq!(c.global(),0);}
#[test]#[should_panic]fn owner_auth_required(){let(e,id)=setup();e.mock_auths(&[]);SlotsClient::new(&e,&id).bump_isolated(&1,&1);}
#[test]#[should_panic]fn overflow_rejected(){let(e,id)=setup();let c=SlotsClient::new(&e,&id);c.bump_isolated(&1,&u64::MAX);c.bump_isolated(&1,&1);}
#[test]#[should_panic]fn out_of_range_slot_rejected(){let(e,id)=setup();SlotsClient::new(&e,&id).bump_shared(&16,&1);}
#[test]#[should_panic]fn zero_increment_rejected(){let(e,id)=setup();SlotsClient::new(&e,&id).bump_isolated(&1,&0);}
}
