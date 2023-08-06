# DevOps Task-2: Dockerization and CI/CD Setup

This repository contains a login application. The goal was to dockerize the application and set up a CI/CD pipeline using GitHub Actions.

# Level 1: Dockerization

Frontend and Backend:
Source code for both frontend and backend is available in their respective directories.
Dockerfiles are present in each directory to containerize the applications.
Docker-compose is used to orchestrate the containers ensuring the frontend container can communicate with the backend containers.
# Level 2: Load Balancing and Reverse Proxy

Multiple Backend Instances:

Two backend containers run simultaneously on different ports.
NGINX is used as a reverse proxy and load balancer to distribute requests in a 60:40 ratio between the two backend servers.
Access Points:

frontend.localhost maps to the frontend container.
backend1.localhost maps to the first backend server.
backend2.localhost maps to the second backend server.
Compression:

The frontend is compressed using NGINX for optimized performance.

# Level 3: CI/CD Pipeline with GitHub Actions

CI Setup:

On every push to the dev branch, the application is built, and Docker images are pushed to Docker Hub.
GitHub Actions are utilized for this Continuous Integration process.
CD Setup:

Every day at 12:00 AM, the code from the dev branch is automatically merged into the master branch.
This ensures that the master branch always has the latest stable version of the application.
Getting Started
Clone the repository:

git clone <repository-url>
Navigate to the project directory:

cd Spider2
Run using Docker Compose:

docker-compose up -d
Access the application:

Open a web browser and navigate to frontend.localhost to access the frontend.
Use backend1.localhost and backend2.localhost to access the two backend servers.
Remember, to view the CI/CD pipeline in action, you can navigate to the "Actions" tab on the GitHub repository.

Note: Ensure you have Docker and Docker Compose installed on your machine to run the application locally.
