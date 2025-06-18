use soroban_sdk::{contracttype, Address, BytesN};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[contracttype]
pub struct ProjectData {
    pub hash: BytesN<32>,
    pub status: ProjectStatusEnum,
}
#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum ProjectStatusEnum {
    NotSet,
    Pending,
    Approved,
    Rejected
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Whitelist(Address),
    ProjectStatus(BytesN<32>),
    ProjectIndex(u32),
    ProjectIndexLength,
}