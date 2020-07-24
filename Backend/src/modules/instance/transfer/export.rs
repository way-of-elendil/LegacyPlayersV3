use crate::modules::live_data_processor::Event;

use crate::modules::armory::Armory;
use crate::modules::instance::dto::{InstanceFailure, InstanceViewerMeta};
use crate::modules::instance::tools::ExportInstance;
use crate::modules::instance::Instance;
use rocket::State;
use rocket_contrib::json::Json;

#[openapi]
#[get("/export/<instance_meta_id>/<event_type>")]
pub fn get_instance_event_type(me: State<Instance>, instance_meta_id: u32, event_type: u8) -> Result<Json<Vec<Event>>, InstanceFailure> {
    me.export_instance_event_type(instance_meta_id, event_type).map(Json)
}

#[openapi]
#[get("/export/<instance_meta_id>")]
pub fn get_instance_meta(me: State<Instance>, armory: State<Armory>, instance_meta_id: u32) -> Result<Json<InstanceViewerMeta>, InstanceFailure> {
    me.get_instance_meta(&armory, instance_meta_id).map(Json)
}
