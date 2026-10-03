use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use bson::{doc, oid::ObjectId};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::Collection;
use rand::rngs::OsRng;

use crate::{
    error::{AppError, AppResult},
    modules::{
        auth::dto::{AuthResponse, Claims, LoginDto, RegisterDto, UserPayload},
        user::model::{Role, User},
    },
    state::AppState,
};

pub struct AuthService;

impl AuthService {
    fn users(state: &AppState) -> Collection<User> {
        state.db.collection::<User>("users")
    }

    /// Register a new user. Returns a JWT + public user payload.
    pub async fn register(state: &AppState, dto: RegisterDto) -> AppResult<AuthResponse> {
        let col = Self::users(state);

        // Check email uniqueness
        if col.find_one(doc! { "email": &dto.email }).await?.is_some() {
            return Err(AppError::Conflict(format!(
                "Email '{}' is already registered",
                dto.email
            )));
        }

        // Hash password with Argon2
        let salt = SaltString::generate(OsRng);
        let hash = Argon2::default()
            .hash_password(dto.password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("Password hashing failed: {e}")))?
            .to_string();

        let now = Utc::now();
        let user = User {
            id: None,
            name: dto.name,
            email: dto.email,
            password_hash: hash,
            role: Role::User,
            avatar_url: None,
            created_at: now,
            updated_at: now,
        };

        let result = col.insert_one(&user).await?;
        let id = result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| AppError::Internal("Failed to get inserted id".into()))?;

        let token = Self::make_token(state, &id, &Role::User)?;

        Ok(AuthResponse {
            token,
            user: UserPayload {
                id: id.to_hex(),
                name: user.name,
                email: user.email,
                role: Role::User,
                avatar_url: None,
            },
        })
    }

    /// Login with email + password. Returns a JWT + public user payload.
    pub async fn login(state: &AppState, dto: LoginDto) -> AppResult<AuthResponse> {
        let col = Self::users(state);

        let user = col
            .find_one(doc! { "email": &dto.email })
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".into()))?;

        let parsed = PasswordHash::new(&user.password_hash)
            .map_err(|e| AppError::Internal(format!("Hash parse error: {e}")))?;

        Argon2::default()
            .verify_password(dto.password.as_bytes(), &parsed)
            .map_err(|_| AppError::Unauthorized("Invalid email or password".into()))?;

        let id = user
            .id
            .ok_or_else(|| AppError::Internal("User has no id".into()))?;

        let token = Self::make_token(state, &id, &user.role)?;

        Ok(AuthResponse {
            token,
            user: UserPayload {
                id: id.to_hex(),
                name: user.name,
                email: user.email,
                role: user.role,
                avatar_url: user.avatar_url,
            },
        })
    }

    /// Build a signed JWT with a 1-hour expiry.
    fn make_token(state: &AppState, id: &ObjectId, role: &Role) -> AppResult<String> {
        let exp = (Utc::now() + chrono::Duration::hours(1)).timestamp() as usize;
        let claims = Claims {
            sub: id.to_hex(),
            role: role.clone(),
            exp,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(state.config.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(format!("Token encoding failed: {e}")))
    }
}
