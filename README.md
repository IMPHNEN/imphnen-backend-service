# IMPHNEN Backend Service

<p align="center">
  <img src="docs/logo.svg" alt="IMPHNEN">
</p>

This repository serves as the **monorepo** for all backend services of IMPHNEN. It encompasses several main services:

1. **IMPHNEN-Backend** - Provides fundamental functionalities and shared resources for other services.
2. **IMPHNEN-IAM** - Handles identity and access management across IMPHNEN applications.
3. **IMPHNEN-CMS** - Supports the cms services by IMPHNEN [Landing Page website](https://imphnen.dev/).
4. **IMPHNEN-Gacha** - Supports the gacha services by IMPHNEN [Gacha website](https://gacha.imphnen.dev/).
5. **IMPHNEN-Dimentorin** - Supports the mentoring services by IMPHNEN [Dimentorin website](https://dimentorin.imphnen.dev/).
6. **IMPHNEN-Gateway** - Acts as the API gateway, routing requests to appropriate services.
7. **IMPHNEN-Middleware** - Acts as the middleware for the API Gateway, providing authentication and authorization.

## How to Install

1. **Clone the repository**:

   ```sh
   git clone https://github.com/IMPHNEN/imphnen-backend-service.git
   cd imphnen-backend-service
   ```

2. **Set up the environment**:

   - Copy the example environment files:

     ```sh
     cp .env.example .env
     ```

     if you use windows based system

     ```sh
     ./apply-env.ps1
     ```

     if you use unix based system

     ```sh
     source ./apply-env.sh
     ```

   - Modify the `.env` files with your specific configuration settings.

3. **Install dependencies**:

   Ensure you have [Rust](https://www.rust-lang.org/) installed. Then, run:

   ```sh
   cargo fetch
   ```

4. **Run the seeders**:

   to run the seeders, run:

   ```sh
   cargo run --bin seeder
   ```

## How to Run

### Development

To run the services in development mode:

1. **Start the database and other dependencies** using Docker Compose:

   ```sh
   docker-compose up -d
   ```

2. **Run using cargo run**. For example, to run the Core Service:

   ```sh
   cargo run --bin api
   ```

3. **Run using cargo watch**. For example, to run the Core Service:

   ```sh
   cargo watch -x "run --bin api"
   ```

### Production

For production deployment:

1. **Build the Docker image**:

   ```sh
   docker build -t imphnen-backend .
   ```

2. **Run the Docker container**:

   ```sh
   docker run --name imphnen-backend -d --env-file .env -p 3000:3000 imphnen-backend
   ```

   Adjust the port and environment variables as needed.

## How to Run the Tests

1. **Run the tests**:

   ```sh
   cargo test -p tests
   ```

## How to Contribute

1. **Fork the repository** and clone it locally.
2. **Create a new branch** for your feature or fix:

   ```sh
   git checkout -b feat/your-feature-name
   ```

3. **Make your changes**, commit them, and push to your forked repository.
4. **Create a pull request** to the `develop` branch of this repository.

If you encounter any issues or have questions, feel free to create a new issue in the repository.

---

_Note: For detailed API documentation, please refer to our [API Docs](https://api.imphnen.dev/docs)._
