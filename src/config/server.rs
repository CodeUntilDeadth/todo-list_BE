use std::sync::LazyLock;
pub static SERVER_PORT: LazyLock<u16> = LazyLock::new(||{
    7070
});