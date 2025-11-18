/// Application layer for authentication module
///
/// This layer contains use cases that orchestrate domain entities
/// and infrastructure services. Use cases are the entry points for
/// all authentication business logic.

pub mod register_user;
pub mod login_user;
pub mod logout_user;
pub mod refresh_token;

// Re-export use cases and commands
pub use register_user::{RegisterUserCommand, RegisterUserUseCase};
pub use login_user::{
    LoginUserUseCase,
    LoginWebCommand,
    LoginApiCommand,
    AuthConfig,
};
pub use logout_user::LogoutUserUseCase;
pub use refresh_token::{RefreshTokenCommand, RefreshTokenUseCase, RefreshConfig};
