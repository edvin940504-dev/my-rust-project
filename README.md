# my-rust-project
Built a backend API system simulating a crypto wallet service with user registration, wallet creation, balance tracking, and transaction handling, using Rust and Actix-web.


# Project Within RustyCore – Kraken Backend

A fast, secure, and extensible cryptocurrency wallet API written in Rust. Built using `actix-web` and `sqlx`, this project demonstrates real-world authentication, wallet management, and transaction handling—perfectly aligned with modern backend needs in the crypto domain.

---

## 🚀 Features

- 🔐 **User Registration & Login** (with Argon2 password hashing)
- 🧾 **Wallet Management** – Create and fetch wallets by user
- 💸 **Transaction API** – Transfer funds between wallets
- 🌐 **RESTful Endpoints** with `actix-web`
- 🗃️ **PostgreSQL Integration** with `sqlx`
- 🛡️ **Secure Architecture** using `.env` configuration & UUIDs

---

## 🧩 Project Structure

kraken_backend/
│
├── src/
│ ├── models/
│ │ ├── mod.rs
│ │ ├── user.rs
│ │ ├── wallet.rs
│ │ └── transaction.rs
│ │
│ ├── routes/
│ │ ├── mod.rs
│ │ ├── user_routes.rs
│ │ ├── wallet_routes.rs
│ │ └── transaction_routes.rs
│ │
│ └── main.rs
│
├── .env
└── Cargo.toml


---

## 🔧 Setup Instructions

### 1. **Clone the Repo**

```bash
git clone https://github.com/edvin940504-dev/my-rust-project.git
cd my-rust-project

2. Create .env file

DATABASE_URL=postgres://postgres:<your-password>@localhost/kraken_db
JWT_SECRET=super_secret_key
SERVER_ADDRESS=127.0.0.1:8080

    Replace <your-password> with your actual local Postgres password.

3. Run Database (PostgreSQL)

Make sure you have a database named kraken_db:

CREATE DATABASE kraken_db;

Then create the necessary tables:

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

CREATE TABLE wallets (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    balance FLOAT NOT NULL
);

4. Run the Server

cargo run

The server will start at:
📍 http://127.0.0.1:8080
📬 API Endpoints
User

    POST /register – Create new user

    POST /login – Authenticate user

Wallets

    POST /wallets – Create wallet for a user

    GET /wallets/{user_id} – List wallets by user

Transactions

    POST /transactions – Transfer between wallets

🛠️ Technologies Used

    Rust 🦀

    actix-web – Web framework

    sqlx – Async Postgres ORM

    Argon2 – Password hashing

    dotenvy – Environment config

    uuid – Unique identifiers

    chrono – Timestamps

📜 License

MIT License. See LICENSE file for details.
🙌 Author

Edvin
GitHub: @edvin940504-dev


---

### 🏗️ **Step 3: Save the file**

Paste the copied content into `README.md` and **save**.

---

### 🏗️ **Step 4: Add, commit, and push to GitHub**

Open your terminal (inside the project folder) and run:

```bash
git add README.md
git commit -m "Add README with project details"
git push