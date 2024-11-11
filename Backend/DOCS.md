# Documentation

This is a temporary documentation file for the backend. Once more of the project gets done we'll use an automated documentation tool like rustdoc to document things.

Then, you'll need to download [Docker Desktop](https://www.docker.com/) if you haven't already. Once you have that running, open a terminal window in the `backend` directory (or `cd` there) and enter `docker compose up -d`. Eventually, you should see that everything has started up.

Then, to test the database, go to `localhost:8080` in your favorite browser. You should see a sign-in window. Select `PostgreSQL` as your system, `db` as your server, `admin` as your username, `password1` as your password, and `admin` as your database. Then, hit login, and you should be taken to a dashboard. You can then use this to do whatever testing you want with the DB.

To test the interface, run `curl localhost:3000` or go to `localhost:3000` in your favorite browser. You should see the message "Hello, World!".

LOGAN-CHANGE: currently, when making changes to rust code, I'm running this command to restart and re-compile the rust code: `docker compose down && docker compose build --no-cache && docker compose up -d`