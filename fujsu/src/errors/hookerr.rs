use dobby_rs::DobbyHookError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HookError {
    #[error(transparent)]
    Dobby(#[from] DobbyHookError),
    
    #[error("Hook returned a Nullpointer trampoline")]
    Null,
    #[error("Paramter {0} is a Nullpointer")]
    Nullpointer(String),
    #[error("Trampoline to {0} is none!")]
    NoTrampoline(String),
    #[error("Failed to hook {0}")]
    Failed(String),
}