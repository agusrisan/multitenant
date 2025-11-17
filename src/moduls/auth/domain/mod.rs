/// Domain layer for authentication module
///
/// This layer contains pure business logic with no external dependencies.
/// Following DDD principles, domain entities enforce business rules and invariants.

pub mod user;
pub mod session;
pub mod token_pair;
pub mod value_objects;

// Re-export main types for convenience
pub use user::{User, UserDto, CreateUserDto};
pub use session::{Session, SessionCookie};
pub use token_pair::{TokenPair, JwtToken, TokenType, Claims};
pub use value_objects::{Email, PasswordHash, CsrfToken};
