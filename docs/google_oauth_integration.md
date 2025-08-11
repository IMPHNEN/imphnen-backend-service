# Google OAuth 2.1 Integration

This document describes the implementation of Google OAuth 2.1 authentication in the backend service using PKCE (Proof Key for Code Exchange) flow with enhanced security measures and robust role management.

## Overview

The Google OAuth integration allows users to authenticate using their Google accounts. The implementation follows OAuth 2.1 best practices with PKCE for public clients and includes several security enhancements and intelligent role assignment.

## Security Features

### 1. PKCE (Proof Key for Code Exchange)
- Uses SHA256 challenge method
- Prevents authorization code interception attacks
- Required for OAuth 2.1 compliance

### 2. CSRF Protection
- Custom signed state tokens for stateless CSRF protection
- Tokens are time-limited (10 minutes)
- Signature verification prevents tampering
- No server-side session storage required

### 3. Input Validation
- Authorization code length and character validation
- State parameter validation
- Protection against injection attacks

### 4. Secure Token Generation
- Separate secrets for access and refresh tokens
- Configurable token expiration times
- Strong random token generation

### 5. Intelligent Role Assignment
- Configurable default role via environment variable
- Automatic role lookup by name ("User")
- Multiple fallback mechanisms to ensure robustness
- Integration with existing role management system

## Configuration

### Environment Variables

```bash
GOOGLE_CLIENT_ID="your_google_client_id"
GOOGLE_CLIENT_SECRET="your_google_client_secret"
GOOGLE_REDIRECT_URL="http://127.0.0.1:8080/api/v1/auth/google/callback"
```

### Role Assignment Strategy

The system automatically assigns the default "User" role to new Google OAuth users using the role ID from the seed data (`5713cb37-dc02-4e87-8048-d7a41d352059`). This ensures consistency with the database schema and eliminates the need for additional configuration.

### Required Scopes

The integration requests minimal required scopes:
- `https://www.googleapis.com/auth/userinfo.email`
- `https://www.googleapis.com/auth/userinfo.profile`

### Obtaining Credentials from Google Cloud Console

1. **Navigate to Google Cloud Console:** Go to the [Google Cloud Console](https://console.cloud.google.com/).
2. **Select/Create a Project:** Choose an existing project or create a new one.
3. **Enable Google People API:** In the navigation menu, go to `APIs & Services` > `Library` and search for "Google People API" and enable it.
4. **Create OAuth Consent Screen:** Go to `APIs & Services` > `OAuth consent screen`.
   - Configure your consent screen, including application name, user support email, and developer contact information.
5. **Create Credentials:** Go to `APIs & Services` > `Credentials`.
   - Click `Create Credentials` > `OAuth client ID`.
   - Select "Web application" as the application type.
   - Provide a name for your OAuth 2.0 client.
   - Under `Authorized redirect URIs`, add the `GOOGLE_REDIRECT_URL` specified in your environment variables (e.g., `http://127.0.0.1:8080/api/v1/auth/google/callback`).
   - Click "Create". Your Client ID and Client Secret will be displayed. Copy these values and set them as `GOOGLE_CLIENT_ID` and `GOOGLE_CLIENT_SECRET` in your environment.

## API Endpoints

### 1. Initiate OAuth Flow
**GET** `/api/v1/auth/google/login`

Redirects user to Google OAuth authorization URL with:
- PKCE code challenge
- Signed CSRF state token
- Required scopes

**Response:** HTTP 302 redirect to Google OAuth

**Example:**
```bash
curl -v http://127.0.0.1:8080/api/v1/auth/google/login
```

### 2. OAuth Callback
**GET** `/api/v1/auth/google/callback?code={code}&state={state}`

Handles the OAuth callback from Google.

**Parameters:**
- `code`: Authorization code from Google
- `state`: CSRF state token (must match the one issued)

**Response:**
```json
{
  "user": {
    "id": "user_id",
    "role": {
      "id": "5713cb37-dc02-4e87-8048-d7a41d352059",
      "name": "User",
      "is_deleted": false,
      "permissions": [
        {
          "id": "permission_id",
          "name": "basic_access",
          "description": "Basic user access"
        }
      ],
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    },
    "email": "user@example.com",
    "fullname": "User Name",
    "phone_number": "",
    "is_active": true,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  },
  "token": {
    "access_token": "jwt_access_token",
    "refresh_token": "jwt_refresh_token"
  }
}
```

**Example:**
```bash
# This would typically be called by Google's redirect
curl -v "http://127.0.0.1:8080/api/v1/auth/google/callback?code=AUTH_CODE&state=CSRF_STATE"
```

## Flow Description

1. **User clicks "Login with Google"**
   - Frontend calls `/api/v1/auth/google/login`
   - Server generates PKCE challenge and signed CSRF state
   - User is redirected to Google OAuth

2. **User authorizes application**
   - Google redirects to callback URL with code and state
   - Server validates CSRF state token signature and expiration
   - Server validates authorization code format

3. **Token exchange**
   - Server exchanges authorization code for Google access token
   - Fetches user profile from Google API
   - Creates or updates user in database

4. **Role assignment for new users**
   - Uses configured `DEFAULT_USER_ROLE_ID` if available
   - Falls back to querying database for "User" role
   - Final fallback to hardcoded User role ID

5. **JWT token generation**
   - Generates access token (15 min expiry)
   - Generates refresh token (1 day expiry)
   - Returns user data with complete role information and tokens

## Error Handling

### Authentication Errors (401)
- Invalid or expired CSRF state token
- Failed token exchange with Google
- Token generation failures

### Validation Errors (400)
- Invalid authorization code format
- Missing or malformed parameters
- CSRF token validation failures

### Server Errors (500)
- Database connection issues
- Google API communication failures
- Role lookup failures (with fallback mechanisms)
- Internal processing errors

## Role Management Integration

### Available Roles
The system integrates with the existing role management system. Current default roles include:
- **User** (`5713cb37-dc02-4e87-8048-d7a41d352059`) - Default role for new users
- **Admin** (`f6b03f25-e416-4893-ac88-caaa690afb07`) - Administrative access
- **Staff** (`50133429-f4b1-4249-9f97-7b86e6ee9d86`) - Staff-level access
- **Mentor** - For mentoring platform features

### Role Assignment Process
1. **Configuration check**: Uses `DEFAULT_USER_ROLE_ID` environment variable
2. **Dynamic lookup**: Queries roles table for "User" role by name
3. **Fallback protection**: Uses seeded User role ID if all else fails
4. **Audit logging**: All role assignment decisions are logged

### Role Permissions
Each role includes associated permissions that control user access within the application. The complete role and permission information is returned in the OAuth callback response.

## Security Considerations

### Implemented ✅
- **PKCE flow**: Prevents code interception attacks
- **CSRF protection**: Signed state tokens with time limits
- **Input validation**: All parameters validated for format and content
- **Minimal scopes**: Only email and profile requested
- **Separate JWT secrets**: Different secrets for access/refresh tokens
- **Token expiration**: Reasonable expiry times (15min/1day)
- **Audit logging**: OAuth events logged for monitoring
- **Secure defaults**: Safe fallbacks for configuration
- **Role-based access**: Integration with permission system
- **Robust role assignment**: Multiple fallback mechanisms

### Additional Recommendations
- Use HTTPS in production environments
- Implement rate limiting for auth endpoints
- Monitor for suspicious OAuth activity patterns
- Regular security audits and dependency updates
- Consider implementing additional 2FA for admin users
- Set up alerts for authentication failures
- Monitor role assignment patterns for anomalies

## Testing

### Prerequisites
1. Valid Google account credentials
2. Properly configured redirect URLs in Google Console
3. Valid environment variables set
4. Database with seeded roles

### Test Cases
1. **Successful OAuth Flow**
   ```bash
   # Step 1: Initiate flow
   curl -v http://127.0.0.1:8080/api/v1/auth/google/login
   
   # Step 2: Follow redirect and complete OAuth
   # Step 3: Verify callback returns valid JWT tokens and user with role
   ```

2. **Role Assignment**
   - Test with configured `DEFAULT_USER_ROLE_ID`
   - Test with missing role configuration (fallback mechanisms)
   - Test with invalid role ID (should use fallback)

3. **CSRF Protection**
   - Test with invalid state token
   - Test with expired state token
   - Test with tampered state token

4. **Input Validation**
   - Test with malformed authorization codes
   - Test with missing parameters
   - Test with oversized parameters

### Monitoring
- Monitor authentication success/failure rates
- Track CSRF validation failures
- Monitor token generation latency
- Alert on unexpected error patterns
- Track role assignment patterns

## Implementation Details

### Key Components

1. **GoogleOauthServiceImpl**
   - Handles OAuth flow logic
   - Manages PKCE and CSRF tokens
   - Integrates with Google APIs
   - Implements intelligent role assignment

2. **get_default_role_id() Helper**
   - Hierarchical role lookup strategy
   - Database integration for dynamic role resolution
   - Robust fallback mechanisms

3. **CSRF Token Module**
   - Generates signed state tokens
   - Validates token signatures and expiration
   - Stateless design (no server storage)

4. **Error Handling**
   - Comprehensive error types
   - Proper HTTP status codes
   - Security-aware error messages

### Dependencies

- `oauth2` - OAuth 2.1 client implementation
- `reqwest` - HTTP client for Google API calls
- `jsonwebtoken` - JWT token generation
- `uuid` - Random ID generation
- `base64` - Token encoding
- `sha2` - Cryptographic hashing for CSRF tokens
- `serde` - JSON serialization/deserialization
- `tracing` - Structured logging
- `anyhow` - Error handling

## Changelog

### Version 1.2 (Current)
- ✅ Improved role assignment with database integration
- ✅ Added intelligent role lookup with fallback mechanisms
- ✅ Enhanced error handling for role resolution
- ✅ Complete role and permission information in responses
- ✅ Robust configuration management

### Version 1.1
- ✅ Added CSRF protection with signed state tokens
- ✅ Implemented comprehensive input validation
- ✅ Added configurable default roles for new users
- ✅ Enhanced error handling with proper HTTP status codes
- ✅ Added audit logging for security events

### Version 1.0
- Initial Google OAuth 2.1 implementation
- Basic PKCE flow support
- JWT token generation
- User creation/update logic

**Note**: Ensure the redirect URL in Google Console matches exactly the `GOOGLE_REDIRECT_URL` environment variable.
