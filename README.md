# RustyTasks
![Version](https://img.shields.io/badge/version-0.1.0-blue.svg?cacheSeconds=2592000)
![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)
![Rust Version](https://img.shields.io/badge/rust-1.55%2B-orange.svg)
![MongoDB](https://img.shields.io/badge/MongoDB-4.4%2B-green.svg)

A command-line interface (CLI) application for managing todo lists with MongoDB backend and Google OAuth authentication.

## Overview

This Todo CLI application allows users to create, manage, and synchronize todo lists across devices. It uses MongoDB for data storage and Google OAuth2 for user authentication, providing a secure and scalable solution for task management.

## Features

- Create and manage multiple todo lists
- Add, complete, and remove tasks
- Filter tasks by completion status
- Google OAuth authentication
- MongoDB backend for data persistence
- Synchronization capabilities (push/pull changes)
- Cross-platform compatibility

## Project Structure

```
RustyTasks/
│
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── commands.rs
│   ├── db.rs
│   ├── auth.rs
│   ├── models.rs
│   └── error.rs
│
├── Cargo.toml
├── Cargo.lock
├── .env
└── README.md
```

## Dependencies

- `clap`: Command line argument parsing
- `tokio`: Asynchronous runtime
- `mongodb`: MongoDB driver
- `oauth2`: OAuth2 authentication
- `dotenv`: Environment variable management
- `thiserror`: Error handling

## Installation

1. Clone the repository:
```
admin@keir> git clone https://github.com/manishyoudumb/RustyTasks
admin@keir> cd RustyTasks
```
2. Install Rust (if not already installed):
```
admin@keir> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
3. Set up environment variables:
Create a `.env` file in the project root with the following:
```
MONGODB_URI=mongodb+srv://<cluster>:<password>@<acct_holder>.1hrcu3o.mongodb.net/<database_name?retryWrites=true&w=majority
GOOGLE_CLIENT_ID=YOUR_ID
GOOGLE_CLIENT_SECRET=YOUR_SECRET_KEY
```
4. Testing all the UNIT TESTS:
```
admin@keir> cargo test
```
![Result](https://github.com/user-attachments/assets/924b1a1d-d279-4435-9319-81a43798664c)

5. Build the Project :
```
admin@keir> cargo build --release
```
6. Adding path to env variables :
```
admin@keir> $Env:PATH += ";$(Get-Location)\target\release"
```
7. Run --help for assistance using the application:
```
admin@keir> todo --help
```

## References

- [Rust Documentation](https://doc.rust-lang.org/book/)
- [MongoDB Rust Driver](https://docs.rs/mongodb/latest/mongodb/)
- [OAuth2 Crate](https://docs.rs/oauth2/latest/oauth2/)
- [Tokio Documentation](https://tokio.rs/docs/overview/)
- [Clap Documentation](https://docs.rs/clap/latest/clap/)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request
