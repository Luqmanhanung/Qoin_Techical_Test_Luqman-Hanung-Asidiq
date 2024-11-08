
# Reqres API Test Suite

This repository contains an automated test suite for testing several endpoints of the [Reqres API](https://reqres.in/) using Katalon Studio. The test suite includes tests for the following endpoints: `GET List Users`, `GET Single User`, `PUT Update`, and `POST Register Successful`. Katalon Studio is used as the automation tool for these API tests.

## Table of Contents

- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [Test Cases](#test-cases)
- [Running the Tests](#running-the-tests)

## Project Structure

```
Project_Qoin_Test/
├── Object Repository/
│   └── ReqresAPI/
│       ├── GET List Users             # Request object for fetching user list
│       ├── GET Single User            # Request object for fetching a single user
│       ├── PUT Update                 # Request object for updating user data
│       └── POST Register              # Request object for registering a user
├── Test Cases/
│   ├── GET List Users                 # Test case for GET List Users
│   ├── GET Single User                # Test case for GET Single User
│   ├── PUT Update                     # Test case for PUT Update
│   └── POST Register 			       # Test case for POST Register Successful
└── Test Suites/
    └── Test All API                   # Test suite that includes all test cases
```

## Getting Started

1. **Install Katalon Studio**: Ensure Katalon Studio is installed on your computer. 
2. **Clone the Repository**: Clone this repository to your local machine using:
   ```bash
   git clone https://github.com/Luqmanhanung/Qoin_Techical_Test_Luqman-Hanung-Asidiq.git
   ```
3. **Open the Project in Katalon Studio**: Open Katalon Studio and load the cloned project.

4. **Set Global Variables**:
   - Add the following variables:

     | Name        | Default Value            | Description                          |
     |-------------|--------------------------|--------------------------------------|
     | `baseUrl`   | `https://reqres.in`      | Base URL for Reqres API              |
     | `userId`    | ``                       | Stores the user ID retrieved from API|
     | `userEmail` | ``     				  | Email address used for registration  |

## Test Cases

### 1. GET List Users
- **Endpoint**: `/api/users?page=1`
- Fetches a list of users on page 1.
- **Validations**:
  - Response status code is `200`.
  - Verifies the structure and content of the user data.

### 2. GET Single User
- **Endpoint**: `/api/users/{user_id}`
- Retrieves details of a specific user by ID.
- **Validations**:
  - Response status code is `200`.
  - Stores the user ID and email in global variables `userId` and `userEmail`.

### 3. PUT Update
- **Endpoint**: `/api/users/{user_id}`
- Updates the data of a specific user.
- **Validations**:
  - Response status code is `200`.
  - Ensures the user’s name and job are successfully updated.

### 4. POST Register
- **Endpoint**: `/api/register`
- Registers a user using an email that exists in Reqres's database (`eve.holt@reqres.in`).
- **Validations**:
  - Response status code is `200`.
  - Ensures the response contains an `id` and `token`, indicating successful registration.

## Running the Tests

1. **Open the Test Suite**: Navigate to `Test Suites` and select `Test All API`.
2. **Run the Test Suite**: Right-click on `Test All API` and select **Run** to execute all test cases.
3. **View Results**: After execution, check the **Log Viewer** for test results. All test cases should pass if the API responds as expected.
