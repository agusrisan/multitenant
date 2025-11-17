# API Documentation

## Overview

Multitenant Auth App provides both Web and API authentication endpoints. The API uses JWT tokens for authentication, while web routes use session-based authentication.

**Base URL**: `http://localhost:3000`

---

## Authentication

### API Authentication (JWT)

API routes use Bearer token authentication:

```
Authorization: Bearer <access_token>
```

### Web Authentication (Session)

Web routes use session cookies with CSRF protection.

---

## API Endpoints

### Authentication Endpoints

#### 1. Register User

Register a new user account.

**Endpoint**: `POST /api/auth/register`

**Request Body**:
```json
{
  "name": "John Doe",
  "email": "john@example.com",
  "password": "SecurePassword123!"
}
```

**Response**: `201 Created`
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIs...",
  "token_type": "Bearer",
  "expires_in": 900
}
```

**Validation Rules**:
- `name`: Required, 1-255 characters
- `email`: Required, valid email format, max 255 characters
- `password`: Required, minimum 8 characters

**Error Responses**:
- `400 Bad Request`: Invalid input
- `409 Conflict`: Email already exists

---

#### 2. Login

Authenticate and receive JWT tokens.

**Endpoint**: `POST /api/auth/login`

**Request Body**:
```json
{
  "email": "john@example.com",
  "password": "SecurePassword123!"
}
```

**Response**: `200 OK`
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIs...",
  "token_type": "Bearer",
  "expires_in": 900,
  "user": {
    "id": "01234567-89ab-cdef-0123-456789abcdef",
    "email": "john@example.com",
    "name": "John Doe",
    "email_verified": false,
    "is_active": true
  }
}
```

**Error Responses**:
- `400 Bad Request`: Invalid input
- `401 Unauthorized`: Invalid credentials

---

#### 3. Refresh Token

Get new access token using refresh token.

**Endpoint**: `POST /api/auth/refresh`

**Request Body**:
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIs..."
}
```

**Response**: `200 OK`
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIs...",
  "token_type": "Bearer",
  "expires_in": 900
}
```

**Error Responses**:
- `401 Unauthorized`: Invalid or expired refresh token

---

#### 4. Logout

Revoke JWT tokens and end session.

**Endpoint**: `POST /api/auth/logout`

**Headers**:
```
Authorization: Bearer <access_token>
```

**Response**: `200 OK`
```json
{
  "message": "Logged out successfully"
}
```

**Error Responses**:
- `401 Unauthorized`: Invalid or missing token

---

### User Profile Endpoints

#### 5. Get Profile

Get current user's profile.

**Endpoint**: `GET /api/user/profile`

**Headers**:
```
Authorization: Bearer <access_token>
```

**Response**: `200 OK`
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "email": "john@example.com",
  "name": "John Doe",
  "bio": "Software developer",
  "avatar_url": "https://example.com/avatar.jpg",
  "email_verified": false,
  "is_active": true,
  "created_at": "2025-01-17T10:30:00Z"
}
```

**Error Responses**:
- `401 Unauthorized`: Invalid or missing token
- `404 Not Found`: User not found

---

#### 6. Update Profile

Update user profile information.

**Endpoint**: `PUT /api/user/profile`

**Headers**:
```
Authorization: Bearer <access_token>
```

**Request Body**:
```json
{
  "name": "John Smith",
  "bio": "Full-stack developer",
  "avatar_url": "https://example.com/new-avatar.jpg"
}
```

**Response**: `200 OK`
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "email": "john@example.com",
  "name": "John Smith",
  "bio": "Full-stack developer",
  "avatar_url": "https://example.com/new-avatar.jpg",
  "email_verified": false,
  "is_active": true,
  "created_at": "2025-01-17T10:30:00Z"
}
```

**Validation Rules**:
- `name`: Required, 1-255 characters
- `bio`: Optional, max 500 characters
- `avatar_url`: Optional, valid URL format

**Error Responses**:
- `400 Bad Request`: Invalid input
- `401 Unauthorized`: Invalid or missing token

---

#### 7. Change Password

Change user password.

**Endpoint**: `PUT /api/user/password`

**Headers**:
```
Authorization: Bearer <access_token>
```

**Request Body**:
```json
{
  "current_password": "OldPassword123!",
  "new_password": "NewPassword456!"
}
```

**Response**: `200 OK`
```json
{
  "message": "Password changed successfully"
}
```

**Validation Rules**:
- `current_password`: Required
- `new_password`: Required, minimum 8 characters

**Error Responses**:
- `400 Bad Request`: Invalid input
- `401 Unauthorized`: Invalid current password or missing token

---

### Health Check

#### 8. Health Check

Check application and database health.

**Endpoint**: `GET /health`

**Response**: `200 OK`
```json
{
  "status": "healthy",
  "database": "connected",
  "timestamp": "2025-01-17T10:30:00Z"
}
```

**Error Responses**:
- `503 Service Unavailable`: Database connection failed

---

## Web Endpoints

### Authentication (Session-based)

#### POST `/web/auth/register`
Register via web form (session-based).

#### POST `/web/auth/login`
Login via web form (session-based).

#### POST `/web/auth/logout`
Logout from web session.

### User Profile (Session-based)

#### GET `/web/user/profile`
View profile page.

#### GET `/web/user/edit`
Edit profile page.

#### GET `/web/user/password`
Change password page.

---

## Error Responses

All error responses follow this format:

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid email format",
    "details": {
      "field": "email"
    }
  }
}
```

### Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `VALIDATION_ERROR` | 400 | Invalid input data |
| `AUTHENTICATION_ERROR` | 401 | Invalid credentials or token |
| `AUTHORIZATION_ERROR` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `CONFLICT` | 409 | Resource already exists |
| `INTERNAL_ERROR` | 500 | Server error |

---

## Rate Limiting

API endpoints are rate-limited to prevent abuse:

- **Login/Register**: 5 requests per minute per IP
- **API Endpoints**: 100 requests per minute per user
- **Refresh Token**: 10 requests per minute per user

When rate limit is exceeded, the API returns `429 Too Many Requests`.

---

## CORS

The API supports CORS for specified origins. Configure allowed origins via the `ALLOWED_ORIGINS` environment variable.

Default (development):
- `http://localhost:3000`
- `http://localhost:5173`

---

## Security Headers

All responses include security headers:

- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `X-XSS-Protection: 1; mode=block`
- `Strict-Transport-Security: max-age=31536000; includeSubDomains`

---

## Token Expiration

- **Access Token**: 15 minutes (900 seconds)
- **Refresh Token**: 7 days (604800 seconds)
- **Session**: 24 hours (86400 seconds)

Use the refresh token endpoint to get a new access token before it expires.

---

## Examples

### cURL Examples

#### Register
```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "email": "john@example.com",
    "password": "SecurePassword123!"
  }'
```

#### Login
```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@example.com",
    "password": "SecurePassword123!"
  }'
```

#### Get Profile
```bash
curl -X GET http://localhost:3000/api/user/profile \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

#### Update Profile
```bash
curl -X PUT http://localhost:3000/api/user/profile \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Smith",
    "bio": "Full-stack developer"
  }'
```

---

## JavaScript/TypeScript Examples

### Using Fetch API

```typescript
// Register
const register = async (name: string, email: string, password: string) => {
  const response = await fetch('http://localhost:3000/api/auth/register', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ name, email, password }),
  });

  if (!response.ok) {
    throw new Error('Registration failed');
  }

  return response.json();
};

// Login
const login = async (email: string, password: string) => {
  const response = await fetch('http://localhost:3000/api/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ email, password }),
  });

  if (!response.ok) {
    throw new Error('Login failed');
  }

  const data = await response.json();
  // Store tokens
  localStorage.setItem('access_token', data.access_token);
  localStorage.setItem('refresh_token', data.refresh_token);

  return data;
};

// Get Profile
const getProfile = async () => {
  const token = localStorage.getItem('access_token');

  const response = await fetch('http://localhost:3000/api/user/profile', {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });

  if (!response.ok) {
    throw new Error('Failed to get profile');
  }

  return response.json();
};
```

---

## Support

For issues or questions, please refer to:
- Development Guide: `docs/development.md`
- Deployment Guide: `docs/deployment.md`
- Architecture Documentation: `docs/architecture.md`
