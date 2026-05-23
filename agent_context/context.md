### Intent
The primary goal of this project is to provide a reliable, containerized Rust command-line application that can scan the filesystem of an external drive and export the results to a text file. This setup is specifically designed to run on a macOS host while executing within an Ubuntu-based Docker container to ensure environment consistency and isolation.

### Approach
1. **Language & Libraries**:
   - **Rust**: Chosen for its performance, safety, and excellent filesystem APIs.
   - **`walkdir`**: Used for efficient recursive directory traversal.
   - **`clap`**: Handles command-line argument parsing (input/output paths).
   - **`chrono`**: Provides UTC timestamps for the scan reports.

2. **Containerization**:
   - **Multi-stage Dockerfile**: 
     - A `builder` stage using the official Rust image to compile the binary.
     - A final stage based on `ubuntu:24.04` to provide a lightweight, CLI-only execution environment.
   - **Docker Compose**: Orchestrates the execution, specifically handling the mounting of macOS `/Volumes` into the container's `/mnt/external` path and mapping the current directory for output retrieval.

3. **Workflow**:
   - The application takes an `--input` directory and an `--output` file path.
   - It iterates through the filesystem, collecting file names, sizes (in bytes), and last modification times.
   - Results are written to a formatted table in the specified `.txt` file.
