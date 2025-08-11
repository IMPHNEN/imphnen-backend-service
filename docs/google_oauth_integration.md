# Google OAuth 2.1 Integration

## Overview

This document describes the integration of Google OAuth 2.1 for user authentication within the application. This feature allows users to sign in using their Google accounts, providing a seamless and secure authentication experience. The process involves initiating an OAuth flow with Google, handling the callback, exchanging authorization codes for tokens, and ultimately generating a JSON Web Token (JWT) for the authenticated user.

## Configuration

To enable Google OAuth 2.1 authentication, the following environment variables must be configured. These variables are loaded into the central `Env` struct in `imphnen-libs`.

*   `GOOGLE_CLIENT_ID`: Your Google OAuth 2.1 Client ID.
*   `GOOGLE_CLIENT_SECRET`: Your Google OAuth 2.1 Client Secret.
*   `GOOGLE_REDIRECT_URL`: The URL to which Google will redirect the user after successful authentication. This must match one of the authorized redirect URIs configured in your Google Cloud Console.

### Obtaining Credentials from Google Cloud Console

1.  **Navigate to Google Cloud Console:** Go to the [Google Cloud Console](https://console.cloud.google.com/).
2.  **Select/Create a Project:** Choose an existing project or create a new one.
3.  **Enable Google People API:** In the navigation menu, go to `APIs & Services` > `Library` and search for "Google People API" and enable it.
4.  **Create OAuth Consent Screen:** Go to `APIs & Services` > `OAuth consent screen`.
    *   Configure your consent screen, including application name, user support email, and developer contact information.
5.  **Create Credentials:** Go to `APIs & Services` > `Credentials`.
    *   Click `Create Credentials` > `OAuth client ID`.
    *   Select "Web application" as the application type.
    *   Provide a name for your OAuth 2.0 client.
    *   Under `Authorized redirect URIs`, add the `GOOGLE_REDIRECT_URL` specified in your environment variables (e.g., `http://127.0.0.1:8080/api/v1/auth/google/callback`).
    *   Click "Create". Your Client ID and Client Secret will be displayed. Copy these values and set them as `GOOGLE_CLIENT_ID` and `GOOGLE_CLIENT_SECRET` in your environment.

## API Endpoints

### 1. Initiate Google OAuth Flow

*   **Endpoint:** `/api/v1/auth/google/login`
*   **Method:** `GET`
*   **Description:** This endpoint initiates the Google OAuth 2.1 authentication flow. When accessed, it generates a Google authorization URL and redirects the user's browser to Google's authentication page. The user will be prompted to grant permissions to your application.

*   **Example `curl` command:**

    ```bash
    curl -v http://127.0.0.1:8080/api/v1/auth/google/login
    ```

    Upon successful execution, this command will return a `302 Found` status with a `Location` header containing the Google authorization URL. Your browser would typically follow this redirect.

### 2. Handle Google OAuth Callback

*   **Endpoint:** `/api/v1/auth/google/callback`
*   **Method:** `GET`
*   **Description:** This endpoint handles the redirect from Google after the user has authenticated and granted permissions. Google sends an authorization `code` and a `state` parameter to this URL. The application then uses this `code` to exchange it for an access token and user information with Google. Upon successful validation and user creation/login, a full `LoginResponse` object is returned, identical to the credential-based login, which includes an access token, refresh token, and user details.

*   **Query Parameters:**
    *   `code` (required): The authorization code provided by Google.
    *   `state` (required): The CSRF token generated during the login initiation.

*   **Example `curl` command (conceptual, as `code` and `state` are dynamic):**

    ```bash
    # This curl command is illustrative. The `code` and `state` values are obtained dynamically
    # from Google's redirect after the user authorizes your application.
    # Replace <AUTHORIZATION_CODE> and <CSRF_STATE> with actual values from the Google redirect.

    curl -v "http://127.0.0.1:8080/api/v1/auth/google/callback?code=<AUTHORIZATION_CODE>&state=<CSRF_STATE>"
    ```

    A successful response will typically return a JSON object containing the generated JWT:

    ```json
    {
      "token": {
        "access_token": "your_access_token_here",
        "refresh_token": "your_refresh_token_here"
      },
      "user": {
        "id": "user_id_here",
        "role": {
          "id": "role_id_here",
          "name": "Role Name",
          "permissions": [],
          "created_at": "2023-01-01T12:00:00Z",
          "updated_at": "2023-01-01T12:00:00Z"
        },
        "fullname": "User Fullname",
        "email": "user@example.com",
        "avatar": "http://example.com/avatar.jpg",
        "phone_number": "1234567890",
        "is_active": true,
        "gender": "Male",
        "birthdate": "2000-01-01T00:00:00Z",
        "created_at": "2023-01-01T12:00:00Z",
        "updated_at": "2023-01-01T12:00:00Z"
      }
    }
    ```

## OAuth Flow Diagram

```mermaid
graph TD
    A[User] --> B{Access /api/v1/auth/google/login};
    B --> C[Backend generates Auth URL];
    C --> D{Redirect to Google Auth Page};
    D --> E[User Authenticates with Google];
    E --> F{Google Redirects to /api/v1/auth/google/callback};
    F --> G[Backend Exchanges Code for Token];
    G --> H[Backend Fetches User Info];
    H --> I{User Exists?};
    I -- Yes --> J[Retrieve User];
    I -- No --> K[Create New User];
    J --> L[Generate JWT];
    K --> L;
    L --> M[Return JWT to User];