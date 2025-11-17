```
auth-app/
├─ Cargo.toml
├─ .env
├─ sqlx-data.json
├─ migrations/
│  ├─ 2025xxxxxx_create_users.sql
│  ├─ 2025xxxxxx_create_sessions.sql
│  └─ 2025xxxxxx_create_jwt_tokens.sql
├─ resources/                 # Frontend (React + Inertia + Tailwind + shadcn)
│  ├─ package.json
│  ├─ tsconfig.json
│  ├─ postcss.config.cjs
│  ├─ tailwind.config.ts      # Tailwind 4 config
│  ├─ vite.config.ts
│  ├─ css/
│  │  └─ app.css
│  ├─ js/
│  │  ├─ app.tsx              # root Inertia/React
│  │  ├─ bootstrap.ts         # register inertia, axios, dll
│  │  ├─ layouts/
│  │  │  ├─ AuthLayout.tsx
│  │  │  └─ AppLayout.tsx
│  │  ├─ components/          # shadcn components (Button, Input, dsb)
│  │  └─ pages/
│  │     ├─ Auth/
│  │     │  ├─ Login.tsx
│  │     │  └─ Register.tsx
│  │     └─ Dashboard/
│  │        └─ Index.tsx
└─ src/
   ├─ main.rs
   ├─ config.rs
   ├─ startup.rs           # build_app(), wiring router utama
   ├─ bootstrap/              # BACKEND BOOTSTRAP
   │  ├─ mod.rs
   │  ├─ app_state.rs         # AppState (db pool, config, dll)
   │  ├─ database.rs          # init Sqlx PgPool
   │  └─ telemetry.rs         # tracing / logging
   ├─ shared/              # shared logic antar modul
   │  ├─ error.rs
   │  ├─ result.rs
   │  └─ types.rs
   └─ moduls/
      ├─ auth/             # modul AUTH (login, register, jwt, session)
      │  ├─ mod.rs
      │  ├─ domain/
      │  │  ├─ mod.rs
      │  │  ├─ user.rs
      │  │  ├─ session.rs
      │  │  ├─ token_pair.rs
      │  │  └─ value_objects.rs   # Email, PasswordHash, CsrfToken, dll
      │  ├─ application/
      │  │  ├─ mod.rs
      │  │  ├─ register_user.rs
      │  │  ├─ login_user.rs
      │  │  ├─ logout_user.rs
      │  │  └─ refresh_token.rs
      │  ├─ infra/
      │  │  ├─ mod.rs
      │  │  ├─ db.rs              # kalau mau modul-specific DB helper
      │  │  ├─ postgres_user_repository.rs
      │  │  ├─ postgres_session_repository.rs
      │  │  └─ postgres_token_repository.rs
      │  ├─ web/                  # route & handler untuk /web/auth/*
      │  │  ├─ mod.rs
      │  │  ├─ routes.rs          # /web/login, /web/register, /web/logout
      │  │  ├─ handlers.rs        # handler Inertia
      │  │  ├─ middleware/
      │  │  │  ├─ session_layer.rs
      │  │  │  └─ csrf_layer.rs
      │  │  └─ view_models.rs     # props Inertia
      │  └─ api/                  # route & handler untuk /api/auth/*
      │     ├─ mod.rs
      │     ├─ routes.rs          # /api/auth/login, /register, /refresh, /logout, /me
      │     ├─ handlers.rs
      │     └─ middleware/
      │        └─ jwt_layer.rs
      │
      └─ user/             # modul USER (profil, manajemen user, dsb)
         ├─ mod.rs
         ├─ domain/
         │  ├─ mod.rs
         │  └─ user_profile.rs    # contoh
         ├─ application/
         │  ├─ mod.rs
         │  └─ update_profile.rs  # contoh
         ├─ infra/
         │  ├─ mod.rs
         │  └─ postgres_user_profile_repository.rs
         ├─ web/
         │  ├─ mod.rs
         │  ├─ routes.rs          # /web/profile dsb
         │  └─ handlers.rs
         └─ api/
            ├─ mod.rs
            ├─ routes.rs          # /api/user/*
            └─ handlers.rs
```