# IMPHNEN Backend Service

<p align="center">
  <img src="docs/logo.svg" alt="IMPHNEN">
</p>

This repository serves as the **monorepo** for all backend services of IMPHNEN. It encompasses several main services:

1. **Core Service** - Provides fundamental functionalities and shared resources for other services.
2. **IAM Service** - Handles identity and access management across IMPHNEN applications.
3. **CMS Service** - Supports the cms services by IMPHNEN [Landing Page website](https://imphnen.dev/).
4. **Gacha Service** - Supports the gacha services by IMPHNEN [Gacha website](https://gacha.imphnen.dev/).
5. **Dimentorin Service** - Supports the mentoring services by IMPHNEN [Dimentorin website](https://dimentorin.imphnen.dev/).
6. **Gateway Service** - Acts as the API gateway, routing requests to appropriate services.

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
     sh apply-env.sh
     ```

   - Modify the `.env` files with your specific configuration settings.

3. **Install dependencies**:

   Ensure you have [Rust](https://www.rust-lang.org/) installed. Then, run:

   ```sh
   cargo build
   ```

## How to Run

### Development

To run the services in development mode:

1. **Start the database and other dependencies** using Docker Compose:

   ```sh
   docker-compose up -d
   ```

2. **Run the desired service**. For example, to run the Core Service:

   ```sh
   cargo run -p imphnen-core-service --bin api
   ```

### Production

For production deployment:

1. **Build the Docker image**:

   ```sh
   docker build -t imphnen-backend-service .
   ```

2. **Run the Docker container**:

   ```sh
   docker run -d --env-file .env -p 8080:8080 imphnen-backend-service
   ```

   Adjust the port and environment variables as needed.

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
