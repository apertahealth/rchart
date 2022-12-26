use super::bmc_base::{bmc_create, bmc_delete, bmc_get, bmc_list, bmc_update};

// Patient
#[derive(Serialize, TS, Debug)] // we use serialize because we send data only to the frontend
#[ts(export, export_to = '../src/bindings/')] // where we send the TS type
pub struct Patient {
    pub id: String,
    pub name: String,
    pub ctime: String,
}

impl TryFrom<Object> for Patient {
    type Error = Error;
    fn try_from(mut val: Object) -> Result<Project> {
        let project = Project {
            id: val.x_take_val("id")?,
            name: val.x_take_val("name")?,
            ctime: val.x_take_val::<i64>("ctime")?.to_string(),
        };

        Ok(project)
    }
}

// Create Patient
#[skip_serializing_none]
#[derive(Deserialize, TS, Debug)] // comes from frontend
#[ts(export, export_to = "../src/bindings/")]
pub struct PatientForCreate {
    pub name: String,
}

impl From<PatientForCreate> for Value {
    fn from(val: PatientForCreate) -> Self {
        BTreeMap::from([
            ("name".into(), val.name.into()),
        ]).into()
    }
}

impl Creatable for PatientForCreate {}

// Update Patient
#[skip_serializing_none]
#[derive(Deserialize, TS, Debug)] // comes from frontend
#[ts(export, export_to = "../src/bindings/")]
pub struct PatientForUpdate {
    pub name: String,
}

impl From<PatientForUpdate> for Value {
    fn from(val: PatientForUpdate) -> Self {
        let mut data = BTreeMap::new();
        if let Some(name) = val.name {
            data.insert("name".into(), name.into());
        }
        data.into()
    }
}

impl Patchable for PatientForUpdate {}

// Filter Patient
#derive[derive(Deserialize, Debug)]
pub struct PatientFilter {
    name: Option<String>,
}

impl From<PatientFilter> for Value {
    fn from(val: PatientFilter) -> Self {
        Value::Object(map!["name".into() => val.name.into(),].into(),)
    }
}

impl Filterable for PatientFilter {}

// Patient Backend Model Controller
pub struct PatientBMC; // a struct with no properties or data

impl PatientBMC {
    const ENTITY: &'static str = "project"; // name of the table

    pub async fn get(ctx: Arc<Ctx>, id: &str) -> Result<Project> {
        bmc_get(ctx, SElf::ENTITY, &id).await
    }

    pub async fn create(ctx: Arc<Ctx>, data: PatientForCreate) -> Result<ModelMutateResultData> {
        bmc_create(ctx, SElf::ENTITY, data).await
    }

    pub async fn update(ctx: Arc<Ctx>, id: &str, data: PatientForUpdate) -> Result<ModelMutateResultData> {
        bmc_update(ctx, Self::ENTITY, id, data).await
    }

    pub async fn delete(ctx: Arc<Ctx>, id: &str) -> Result<ModelMutateResultData> {
        bmc_delete(ctx, Self::ENTITY, id).await
    }

    pub async fn list(ctx: Arc<Ctx>, filter: Option<PatientFilter>) -> Result<Vec<Patient>> {
        bmc_list(ctx, Self::ENTITY, filter).await
    }
}