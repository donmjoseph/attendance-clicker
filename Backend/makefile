# make file documentation for reference:
# https://www.gnu.org/software/make/manual/make.html#Errors

# by default, step and re-start the containers
all: run_docker

# just stop the docker containers with `make stop`
stop:
	docker compose down

# stop containers, then build and re-start them
run_docker:
	docker compose down
	docker compose build --no-cache
	docker compose up -d

# clean (remove) cargo's target directory for compiles,
# if you need to do that for some reason
# if you get a lot of errors, try running with sudo
clean:
	rm -rf interface/target/

# completely remove containers, volumes, and networks
purge:
	docker compose down --volumes --remove-orphans

# for a true fresh start
#   completely remove containers, volumes, networks
#   and remove all unused data
# !! CAREFUL !!
#   This may cause unwanted side effects, like removing
#   dependencies for other docker containers on your system.
deep_purge:
	docker compose down --volumes --remove-orphans
	docker system prune --all --volumes --force
