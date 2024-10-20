# Documentation

This is a temporary documentation file for the backend. Once more of the project gets done we'll use an automated documentation tool like rustdoc to document things.

First, create a file `Backend/postgres/pwd.txt`. Put whatever you want in there as a password.

Then, you'll need to download [Docker Desktop](https://www.docker.com/) if you haven't already. Once you have that running, open a terminal window in the `backend` directory (or `cd` there) and enter `docker compose up`. Eventually, you should see that the database has started up (and you'll probably have to press Ctrl+C to exit because reasons). Then, you can enter `docker compose down`, which stops the docker containers.
