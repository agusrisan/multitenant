# Phase 1: Project Setup & Configuration

**Status**: 🔴 Not Started
**Priority**: 🔥 CRITICAL
**Estimated Time**: 4-6 hours
**Dependencies**: None (starting point)

## Overview
Initial project setup including Rust workspace, frontend tooling, database configuration, and development environment.

---

## 1. Rust Backend Setup

### 1.1 Cargo.toml Configuration
- [ ] **Create root Cargo.toml** 📁 `Cargo.toml`
  - **Priority**: High
  - **Complexity**: Simple (30 min)
  - **Dependencies**: None
  - **Tech Stack**:
    - `axum = "0.8.7"` - Web framework
    - `axum_inertia = "0.9.0"` - Inertia.js adapter
    - `sqlx = { version = "0.8.6", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "migrate"] }`
    - `tokio = { version = "1", features = ["full"] }`
    - `tower = "0.5"`
    - `tower-http = { version = "0.6", features = ["fs", "trace", "cors"] }`
    - `serde = { version = "1.0", features = ["derive"] }`
    - `serde_json = "1.0"`
    - `uuid = { version = "1.11", features = ["v7", "serde"] }` - UUID v7 native support
    - `chrono = { version = "0.4", features = ["serde"] }`
    - `bcrypt = "0.15"`
    - `jsonwebtoken = "9"`
    - `tracing = "0.1"`
    - `tracing-subscriber = { version = "0.3", features = ["env-filter"] }`
    - `dotenvy = "0.15"`
    - `anyhow = "1.0"`
    - `thiserror = "1.0"`
    - `validator = { version = "0.18", features = ["derive"] }`
  - **Notes**: Use workspace if planning multi-crate structure

### 1.2 Environment Configuration
- [ ] **Create .env file** 📁 `.env`
  - **Priority**: High
  - **Complexity**: Simple (15 min)
  - **Dependencies**: None
  - **Required Variables**:
    ```env
    DATABASE_URL=postgres://postgres:password@localhost:5432/multitenant_db
    POSTGRES_VERSION=18

    # Server
    HOST=127.0.0.1
    PORT=3000

    # JWT
    JWT_SECRET=your-secret-key-change-in-production
    JWT_ACCESS_EXPIRY=900  # 15 minutes
    JWT_REFRESH_EXPIRY=604800  # 7 days

    # Session
    SESSION_SECRET=your-session-secret

    # CSRF
    CSRF_SECRET=your-csrf-secret

    # Environment
    RUST_LOG=debug
    RUST_ENV=development
    ```

- [ ] **Create .env.example** 📁 `.env.example`
  - **Priority**: Medium
  - **Complexity**: Simple (5 min)
  - **Dependencies**: Requires .env
  - **Notes**: Template without sensitive values

### 1.3 Git Configuration
- [ ] **Create .gitignore** 📁 `.gitignore`
  - **Priority**: High
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **Must Ignore**:
    ```
    /target
    .env
    .idea/workspace.xml
    .idea/tasks.xml
    node_modules/
    resources/dist/
    *.log
    .DS_Store
    ```

---

## 2. Database Setup

### 2.1 SQLx Configuration
- [ ] **Create sqlx-data.json** 📁 `sqlx-data.json`
  - **Priority**: Medium
  - **Complexity**: Simple (10 min)
  - **Dependencies**: Requires migrations to be created
  - **Command**: `cargo sqlx prepare`
  - **Notes**: Generated after first migration, for compile-time verification

### 2.2 Migrations Directory
- [ ] **Create migrations directory** 📁 `migrations/`
  - **Priority**: High
  - **Complexity**: Simple (5 min)
  - **Dependencies**: None
  - **Notes**: SQLx will scan this directory

---

## 3. Frontend Setup

### 3.1 Node.js Configuration
- [ ] **Create package.json** 📁 `resources/package.json`
  - **Priority**: High
  - **Complexity**: Medium (45 min)
  - **Dependencies**: None
  - **Tech Stack**:
    ```json
    {
      "name": "auth-app-frontend",
      "private": true,
      "type": "module",
      "scripts": {
        "dev": "vite",
        "build": "tsc && vite build",
        "preview": "vite preview"
      },
      "dependencies": {
        "react": "^19.0.0",
        "react-dom": "^19.0.0",
        "@inertiajs/react": "^2.0.0",
        "axios": "^1.7.0"
      },
      "devDependencies": {
        "@types/react": "^19.0.0",
        "@types/react-dom": "^19.0.0",
        "@vitejs/plugin-react": "^4.3.0",
        "typescript": "^5.6.0",
        "vite": "^6.0.0",
        "tailwindcss": "^4.0.0",
        "postcss": "^8.4.0",
        "autoprefixer": "^10.4.0"
      }
    }
    ```

### 3.2 TypeScript Configuration
- [ ] **Create tsconfig.json** 📁 `resources/tsconfig.json`
  - **Priority**: High
  - **Complexity**: Simple (20 min)
  - **Dependencies**: None
  - **Config**:
    ```json
    {
      "compilerOptions": {
        "target": "ES2022",
        "useDefineForClassFields": true,
        "lib": ["ES2022", "DOM", "DOM.Iterable"],
        "module": "ESNext",
        "skipLibCheck": true,
        "moduleResolution": "bundler",
        "allowImportingTsExtensions": true,
        "resolveJsonModule": true,
        "isolatedModules": true,
        "noEmit": true,
        "jsx": "react-jsx",
        "strict": true,
        "noUnusedLocals": true,
        "noUnusedParameters": true,
        "noFallthroughCasesInSwitch": true,
        "baseUrl": ".",
        "paths": {
          "@/*": ["./js/*"]
        }
      },
      "include": ["js"],
      "references": [{ "path": "./tsconfig.node.json" }]
    }
    ```

- [ ] **Create tsconfig.node.json** 📁 `resources/tsconfig.node.json`
  - **Priority**: Medium
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **Config**:
    ```json
    {
      "compilerOptions": {
        "composite": true,
        "skipLibCheck": true,
        "module": "ESNext",
        "moduleResolution": "bundler",
        "allowSyntheticDefaultImports": true
      },
      "include": ["vite.config.ts"]
    }
    ```

### 3.3 Vite Configuration
- [ ] **Create vite.config.ts** 📁 `resources/vite.config.ts`
  - **Priority**: High
  - **Complexity**: Medium (30 min)
  - **Dependencies**: Requires package.json
  - **Tech**: Vite 6 with Inertia integration
  - **Key Features**:
    - React plugin
    - Path aliases (@/ → ./js/)
    - Proxy to Rust backend (http://localhost:3000)
    - Build output to resources/dist/

### 3.4 Tailwind Configuration
- [ ] **Create tailwind.config.ts** 📁 `resources/tailwind.config.ts`
  - **Priority**: High
  - **Complexity**: Medium (30 min)
  - **Dependencies**: None
  - **Tech**: Tailwind CSS 4
  - **Config**:
    ```typescript
    import type { Config } from 'tailwindcss'

    export default {
      content: [
        "./js/**/*.{ts,tsx}",
      ],
      theme: {
        extend: {},
      },
      plugins: [],
    } satisfies Config
    ```

- [ ] **Create postcss.config.cjs** 📁 `resources/postcss.config.cjs`
  - **Priority**: Medium
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **Plugins**: tailwindcss, autoprefixer

### 3.5 Base CSS
- [ ] **Create app.css** 📁 `resources/css/app.css`
  - **Priority**: Medium
  - **Complexity**: Simple (15 min)
  - **Dependencies**: None
  - **Content**:
    ```css
    @import "tailwindcss";

    /* Tailwind 4 syntax */
    @layer base {
      :root {
        --background: 0 0% 100%;
        --foreground: 222.2 84% 4.9%;
        /* shadcn color variables */
      }
    }
    ```

---

## 4. Development Tools

### 4.1 Docker Configuration (Optional)
- [ ] **Create docker-compose.yml** 📁 `docker-compose.yml`
  - **Priority**: Low
  - **Complexity**: Medium (30 min)
  - **Dependencies**: None
  - **Services**:
    - PostgreSQL 18
    - pgAdmin (optional)
  - **Notes**: For local database development

### 4.2 Scripts
- [ ] **Create run script** 📁 `scripts/dev.sh`
  - **Priority**: Low
  - **Complexity**: Simple (15 min)
  - **Dependencies**: None
  - **Purpose**: Concurrent Rust + Vite dev servers

---

## Validation Checklist

- [ ] `cargo check` passes
- [ ] `npm install` completes successfully
- [ ] Database connection works (can run migrations)
- [ ] Vite dev server starts without errors
- [ ] All config files are in .gitignore

---

## Next Phase
➡️ **Phase 2: Backend Foundation** - Bootstrap, AppState, database pool, shared utilities

## Notes
- UUID v7 provides time-ordered IDs natively (better than v4 for databases)
- PostgreSQL 18 has improved performance for UUID operations
- Tailwind 4 uses new @import syntax
- React 19 has improved Suspense and Server Components support (even though we're using Inertia)
