# Documentation

This is a temporary documentation file for the backend. Once more of the project gets done we'll use an automated documentation tool like rustdoc to document things.

## Table of Contents

- [Backend Overview](#backend-overview) - All about what our Backend does, how it works, and what it uses
- [Setup Docker](#setup-docker) - How to setup docker on your machine for running locally
- [Running with Makefile](#running-with-makefile) - How to run pre-made commands with Make
- [Interacting Manually via Curl](#interating-manually-via-curl) - How to send GET and POST requests to the databse manually
- [Backend File Structure](#backend-file-structure) - What's where and why
- [Ports](#ports) - What services are assigned to what ports, for your reference

## Backend Overview

The purpose of the Backend is to manage the database storing data about classes, users, and questions/attendance, as well as handle requests.

## Setup Docker

Installing Docker is pretty straight forward. There are two main ways you can install it:
- Docker Desktop: A GUI all-in-one, from Docker's website homepage: [https://www.docker.com/](https://www.docker.com/)
- Docker Engine: Command-line only version from the engine install manual: [https://docs.docker.com/engine/install/](https://docs.docker.com/engine/install/)

Once you have Docker Desktop or Engine installed, you can now begin running the backend locally.

## Running with Makefile

If you've never used the `make` build automation tool before, a quick 2-minute video ["Learn make in 60 seconds." by Jacob Sorber on YouTube](https://www.youtube.com/watch?v=a8mPKBxQ9No) is a good briefer on how to make a makefile and then run the targets.

> side note: If you're on mac or linux, these commands should just work. If you're on windows, install linux. ... Or, try using WSL

In this project's case, there's a file `makefile` which defines some custom, pre-made macros/scripts that can be used to easily run, stop, and manage docker instances (and other things). Here are commands you can run (targets we've defined) and what they do:

- `make` - runs the `all` target:
- `make all` - run the `run_docker` target:
- `make run_docker` - stops any already running containers, builds the containers, and starts the services
- `make stop` - stops any running containers
- `make clean` - removes cargo's target directory made inside the interface folder, when cargo builds the backend. May need to be ran with sudo if many errors occur
- `make purge` - Removes (containers, volumes and network configs) related to the backend, from your computer. A sort of fresh start
- `make deep_purge` - **!! MAY CAUSE UNWANTED SIDE EFFECTS !!** - completely remove containers, volumes, networks, and unused docker data. For **ALL** containers and volumes on your computer. Please be careful.

I would encourage anyone to actually look at the makefile to see what commands are being run, so you can 1) see how they work and 2) make sure they only do things you would want them to on your computer.

## Interating Manually via Curl

If you would like to interact with your local backend instance (either because you don't have access to frontend or frontend isn't able to interact with backend yet), you can easily use the `curl` command line tool to send requests and receive responses.

Curl can be a complicated tool, but here are some simple example, starter, commands you can try:

- `curl 0.0.0.0:3000`: Send an empty request to backend, currently you receive a healthcheck response
- > [ADD MORE]

## Backend File Structure

Here's the general structure of the Backend, and any notes:

```
Backend                         │ Notes
└───.vscode                     │ 
│   │   settings.json           │ (any custom settings for vs code)
│                               │
└───interface                   │ Main rust project
│   └───src                     │ rust code
│   │   └───api_funcs           │
│   │   │   │   GET.rs          │
│   │   │   │   POST.rs         │
│   │   │                       │
│   │   │   api_funcs.rs        │
│   │   │   handler.rs          │
│   │   │   main.rs             │
│   │   │   schema.rs           │
│   │                           │
│   │   target                  │ Cargo-generated folder
│   │   Cargo.lock              │ Cargo-generated file
│   │   Cargo.tomp              │ Rust Project and Dependencies
│   │   Dockerfile              │ Docker setup script for the backend-interface-1 service
│                               │
│   api-request-URIs.md         │ Temp notes file
│   DOCS.md                     │
│   makefile                    │
│   notes.txt                   │ Temp notes file
```

## Ports

- `3000`: backend interface
- `8080`: adminer, gui for viewing/managing database(s)
- `5432 -> 6500`: postgres, shouldn't be accessed
