## Setup

1. First, make sure you have Rust installed on your system.
2. Clone this repository to your local machine.
3. Navigate to the project directory in your terminal.

## Database Configuration

Before running the application, you need to specify the database URL in `db.rs`. This file contains the database connection configuration.

// db.rs
```
.connect("postgres://postgres:your_password_here@localhost/database")
```
Replace "your_password_here" with the password for your PostgreSQL database user. This configuration assumes that your database is named database and is running on localhost. Adjust the URL as necessary to match your actual database configuration.

Creating Necessary Tables
To create the necessary tables in your database, follow the steps below:
1.	Open your PostgreSQL client (e.g., psql) and connect to your database.
2.	Copy the contents of the setupDB.sql file located in this repository.
3.	Paste the SQL script into your PostgreSQL client and execute it to create the required tables.
Here is an example command to execute the SQL script in psql:
```
psql -U postgres -d database -a -f setupDB.sql
```
Replace postgres with your PostgreSQL username, database with the name of your database, and setupDB.sql with the path to your SQL script if it's located in a different directory.
After executing the SQL script, your database will be set up with the necessary tables for the application to function properly.


## API Endpoints
The following API endpoints are available:

- **GET /items: Retrieves all items from the database.
- **GET /item/{id}: Retrieves the item with the specified ID.
- **POST /save: Saves an item to the database.
- **DELETE /item/{id}: Removes the item with the specified ID from the database.
  
Running the Application
To run the application, use the following command:
cargo run
This will compile and run the backend server. You can then access the API endpoints using your preferred HTTP client.

## Contributing
Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or create a pull request.

## License
This project is licensed under the MIT License.
