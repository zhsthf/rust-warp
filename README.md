
# Rust Warp JWT Project

This project is a Rust web application using Warp for the web server and JWT (JSON Web Token) for authentication. It uses MongoDB as the database, running in a Docker container.

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- Docker and Docker Compose (https://docs.docker.com/get-docker/ and https://docs.docker.com/compose/install/)
- (Optional) MongoDB Compass for database inspection (https://www.mongodb.com/products/compass)

## Setting Up the Project

1. **Clone the Repository**

   ```bash
   git clone https://github.com/zhsthf/rust-warp.git
   cd rust-warp
   ```

2. **Set Up Environment Variables**

   Create a `.env` file in the project root and set the necessary environment variables:

   ```plaintext
   JWT_SECRET=your_jwt_secret_here
   MONGO_INITDB_ROOT_USERNAME=mongoadmin
   MONGO_INITDB_ROOT_PASSWORD=secret1
   ```

   Replace `your_jwt_secret_here`, `mongoadmin`, and `secret` with your own values. 

   You can generate a secure JWT secret using various tools. For instance, in Unix/Linux, you can use:

   ```bash
   openssl rand -base64 128
   ```

3. **Start MongoDB with Docker Compose**

   Use Docker Compose to start a MongoDB instance:

   ```bash
   sudo docker compose up -d
   ```

   This will start MongoDB in a Docker container as configured in `docker-compose.yml`.

4. **Run the Application**

   Build and run the Rust application:

   ```bash
   cargo run
   ```

   Your application should now be running and connected to the MongoDB instance.

## Usage

- Access the application through the specified port (default is `8000`).
- Use endpoints such as `/signup`, `/login`, `/user`, and `/admin` for corresponding functionalities.

## Additional Information

- Ensure to keep your JWT secret and MongoDB credentials secure.
- Adjust your MongoDB configuration in `docker-compose.yml` as needed for your environment.
