#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InstanceViewerAttempt {
    pub id: u32,
    pub is_kill: bool,
    pub creature_id: u64,
    pub npc_id: u32,
    pub start_ts: u64,
    pub end_ts: u64,
}
