#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DigitalTimerFromDB {
    pub id: i64,
    pub name: String,
    pub pin: i64,
    pub hour: i64,
    pub minute: i64,
    pub duration: i64,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DigitalTimer {
    pub id: Option<i64>,
    pub name: String,
    pub pin: u8,
    pub hour: u8,
    pub minute: u8,
    pub duration: u32,
    pub is_enabled: bool,
}

impl From<DigitalTimerFromDB> for DigitalTimer {
    fn from(digital_timer: DigitalTimerFromDB) -> Self {
        DigitalTimer {
            id: Some(digital_timer.id),
            name: digital_timer.name,
            pin: digital_timer.pin as u8,
            hour: digital_timer.hour as u8,
            minute: digital_timer.minute as u8,
            duration: digital_timer.duration as u32,
            is_enabled: digital_timer.is_enabled,
        }
    }
}