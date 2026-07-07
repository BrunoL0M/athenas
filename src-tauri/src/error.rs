use thiserror::Error;

/// Error unificado de la app, se irá ampliando con variantes por módulo (db, scanner, git).
#[allow(dead_code)] // quitar cuando el primer módulo lo use
#[derive(Debug, Error)]
pub enum AppError {
    #[error("unknown error: {0}")]
    Unknown(String),
}
