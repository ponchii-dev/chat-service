# Real-Time Chat Service (Rust)

A real-time messaging service built in Rust using WebSockets and Tokio, designed to handle concurrent client connections and scalable message broadcasting.

## Overview

This service implements a WebSocket-based chat system with support for multiple concurrent users, message routing, and presence tracking. The architecture is designed with clear separation of concerns to support future scaling into a distributed system.

## Features

- WebSocket-based real-time messaging
- Concurrent connection handling using Tokio
- Message broadcasting across clients and rooms
- Presence tracking (online users)
- Modular architecture (connection handling, message routing, broker layer)
- Docker support for local development

## Architecture

The system is structured into separate components:

- **WebSocket Layer (`ws.rs`)**  
  Handles client connections, message input/output, and connection lifecycle

- **Broker (`broker.rs`)**  
  Responsible for routing and broadcasting messages between connected clients

- **Presence (`presence.rs`)**  
  Tracks active users and connection state

- **Database (`db.rs`, `models.rs`)**  
  Handles persistence and data models

- **Config / Telemetry**  
  Environment configuration and logging setup

### Message Flow

Client → WebSocket → Broker → Broadcast → Clients

## Running the Project

### Using Docker

```bash
docker-compose up --build