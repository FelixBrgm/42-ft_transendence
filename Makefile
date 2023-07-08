NAME:=transcendence 

all: $(NAME)

$(NAME):
	echo Not implemented yet

postgres:
	cd database && docker-compose up -d

http:
	set -a; . ./database/.env; set +a && cd server/http && cargo run