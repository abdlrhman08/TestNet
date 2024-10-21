## Overview
This project is a Real-Time Testing Dashboard designed to streamline and monitor the automated testing process in DevOps pipelines. It provides users with real-time updates on the status of their projects, allowing them to view logs and results for each test stage as they occur. Leveraging WebSocket technology, the dashboard enables efficient and immediate communication between the server and client, ensuring users receive live feedback on their testing processes.

## Features
1. Real-Time Log Streaming: View logs for each test stage as they are generated, providing immediate insight into the testing process.
2. Project Status Monitoring: Monitor the status of multiple projects simultaneously, with clear indicators for success, failure, and running status.
3. User-Friendly Interface: An intuitive and responsive user interface that organizes project information effectively.
4. WebSocket Integration: Utilizes WebSockets for efficient communication between the server and client, reducing latency and enhancing performance.

### Technologies Used
#### Frontend:

1. React
2. FontAwesome (for icons)
3. CSS for styling

## Backend:

1. Axum(Rust), Tokio and low-level OS primitives
2. WebSocket for real-time communication
3. Any necessary DevOps tools (e.g., Docker, Jenkins)

## Usage
Add Projects: Users can add their projects by providing the necessary webhook URL.
Monitor Tests: As tests are executed, users can monitor the progress through the dashboard. Each project's logs and status updates will be displayed in real-time.
View Logs: Click on each project to expand and view detailed logs for each testing stage
