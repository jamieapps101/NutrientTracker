pub struct Users {
    uid: u32,
    name: String,
    password_hash: String,
}

pub struct Time {
    second: u32,
    minute: u32,
    hour: u32,
}

pub struct Date {
    day: u32,
    month: u32,
    year: u32,
}

pub struct NutrientTargets {
    /// Users Table UID
    user: u32,
    /// Nutrient Table UID
    target_nutrients: u32,
    date_begin: Date,
    /// may not be needed if this can be quickly reconstructed by looking at the proceeding macro date begin
    date_end: Date,
}

pub struct Units {
    uid: u32,
    name: String,
    abbreviation: String,
}

pub struct Nutrients {
    uid: u32,
    calories: f32,
    carbs: f32,
    protein: f32,
    fat: f32,
    source: String,
}

pub struct Consumable {
    uid: u32,
    name: String,
    notes: String,
    portion_amount: f32,
    /// Units Table UID
    portion_unit: u32,
    /// Nutrients Table UID
    nutrients: u32,
}

pub struct CompositeConsumable {
    uid: u32,
    name: String,
    notes: String,
    portion_amount: f32,
    /// (Units Table UID)
    portion_unit: u32,
}

pub struct CompositeConsumableNutrients {
    /// used to refer to the composite consumable that these nutrients belong to
    /// (Composite Consumable Table UID)
    composite_consumable_id: u32,
    /// (Consumable Table UID)
    /// Used to refer to the nutrients that the owning consumabled comprises. Mutually exclusive with Consumable attribute.
    consumable: Option<u32>,
    /// Used to refer to the nutrients that the owning consumabled comprises. Mutually exclusive with Consumable attribute.
    /// (Composite Consumable Table UID)
    composite_consumable: Option<u32>,
    scaling: f32,
}

pub struct ConsumptionRecord {
    /// (Users Table UID)
    user: u32,
    /// (Consumable table id)
    consumable: Option<u32>,
    /// (Composite Consumable table id)
    composite_consumable: Option<u32>,
    time: Time,
    /// possibly a single time-date field depending on DB
    date: Date,
}

/// struct to contain incoming data and annotate with a session ID
pub struct Transmission<T> {
    session_id: u32,
    data: T,
}

pub struct LoginRequest {
    username: String,
    password_hash: String,
}

pub enum LoginReply {
    NoUserName,
    IncorrectPassword,
    /// Success option contains a session ID
    Success(u32),
}
