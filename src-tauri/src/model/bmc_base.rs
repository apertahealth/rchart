// reuseable low level backend model controller functions

use super::{fire_model_event, ModelMutateResultData};

pub async fn bmc_get<E>(ctx: Arc<Ctx>, _entity: &'static strc, id: &str)
    -> Result<E> where E: TryFrom<Object, Error = Error>, {
        ctx.get_store().exec_get(&id).await?.try_into()
}

pub async fn bmc_create<D>() -> Result<ModelMutateResultData> where D: Creatable, {}

pub async fn bmc_update<D>() -> Result<ModelMutateResultData> where D: Patchable, {}