# What is Cloudsdale?

Cloudsdale is a CTF (Capture The Flag) platform built with Rust and uses a Jeopardy-style format. It is extremely lightweight and can be quickly deployed using a _simple_ configuration file.

The project draws inspiration from CTFd, Cardinal, GZ::CTF, and Ret2Shell, combining the best of each to create this project. Due to the author's unique and exaggerated understanding of software, as well as the limited resources of the author's school, the aim is to create a lightweight and user-friendly CTF platform to provide a great experience for the school's CTF team.

## Use Cases

Just like ACM, CTF should have its own customized platform. Cloudsdale is very suitable for organizing small-scale CTF games or for CTF team training. The flexible challenge management function can save and use challenges more efficiently.

## Features

- Challenges
    - Static Challenges: No target machine, the grading relies on one or more known flag strings, usually dependent on the attachment system.
    - Dynamic Challenges: Dynamic target machines, the grading can rely on static flag strings or dynamically generated flags (usually a `UUID`).
- Target Machines
    - Multi-port support
    - Customizable basic environment variables for custom images
    - Customizable container resource requests (memory and CPU)
    - Customizable flag injection variable names
    - Optional port mapping mode
    - Traffic capture implemented through platform proxy
- Competitions
    - Customizable challenge scores
    - Customizable first, second, and third blood reward ratios
    - Ability to disable/enable challenges at any time during the competition, allowing for multiple releases of challenges
    - Competition message broadcasting based on Websocket
- Database
    - Support for multiple relational databases via SeaORM (PostgreSQL, SQLite3, MySQL)
- Container Support
    - Docker
    - Kubernetes
