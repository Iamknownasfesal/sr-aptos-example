use strum::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Events {
    NewPoolEvent,
}


impl Events {
    pub fn as_str(&self) -> &'static str {
        match self {
            Events::NewPoolEvent => "0x02a9d1afd0f2053e1eb569fde03b8407ff9d9eb0e6d76d88c19ec3d5424eafa9::events::NewPool",
        }
    }
}
