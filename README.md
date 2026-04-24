# Project Title

Event Tickets – A Soroban Smart Contract for Concert and Event Ticket Management on Stellar

## Project Vision

This project demonstrates how to build a **ticket management system on Stellar** using Soroban smart contracts. It provides a decentralized way to create events, manage ticket sales, and verify ownership on-chain.

---

## Description

A Soroban smart contract dApp that allows administrators to create events with a set number of tickets at a specific price, and enables users to purchase tickets directly on the Stellar blockchain. Designed for managing concert, conference, and event tickets with full on-chain transparency.

---

## Features

### 1. Event Creation
- Admin creates events with unique IDs
- Set total ticket count and price per event
- On-chain event data stored permanently

### 2. Ticket Purchase
- Users can buy tickets for specific events
- One ticket per address per event enforced
- Available ticket count decrements on purchase

### 3. Ownership Verification
- Query ticket count remaining for any event
- Verify if a specific address owns a ticket
- Full on-chain transparency for ticket distribution

### 4. Beginner-Friendly
- Clear, readable Rust code for Soroban
- Minimal, working example for learning

---

## Contract Functions

- **create_event(event_id, total_tickets, price)** – Admin creates a new event
- **buy_ticket(event_id, buyer)** – User purchases a ticket
- **get_ticket_count(event_id)** – Returns remaining tickets
- **get_ticket_owner(event_id, buyer)** – Returns true if address owns a ticket

---

## Contract

- **Contract ID**: [CCQREUTWSNE2GLSMKWGTEEMLHDOHVTSD4JBOWGTT6YOYBEEUWQXUYFSS](https://stellar.expert/explorer/testnet/tx/72533a2a0b09dd203b9c95bab8f1be6ac3f6cdd7074f6520d3b6a6e7780bb0eb)

![screenshot](https://i.ibb.co/tPnh0PHr/image.png)

---

## Future Scopes

### 1. Ticket Transfer
- Allow ticket owners to transfer tickets to other addresses

### 2. Refund Mechanism
- Add admin-controlled refund functionality before event date

### 3. Multiple Tickets Per Buyer
- Support purchasing multiple tickets in a single transaction

### 4. Event Metadata
- Add event name, date, and venue details stored on-chain

### 5. Frontend dApp
- Build a web interface for browsing events and purchasing tickets

### 6. Ticket Resale
- Implement a peer-to-peer ticket resale marketplace with price caps

---

## Profile

- **Name:** rolandqsloanu
