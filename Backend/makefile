# make file documentation for reference:
# https://www.gnu.org/software/make/manual/make.html#Errors

# by default, stop and re-start the containers
all: run

# stop containers, then re-build and re-start them
run: stop
	docker compose build --no-cache
	docker compose up -d

# follow `backend-intereface-1`'s log
follow_logs:
	docker logs --follow backend-interface-1

# stop containers, restart, and follow `backend-intereface-1`'s log
runf: run follow_logs

# stop running containers
stop:
	docker compose down

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
deep_purge: purge
	docker system prune --all --volumes --force
