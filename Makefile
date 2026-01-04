# FILE = docker/docker-compose.yml
FILE = docker/minio.docker-compose.yml

DATA_DIR = docker/data

# ======================== System ========================  
clean:system volume
	@echo "🧹 Docker cleanup completed."

system: 
	docker system prune -a -f
volume:
	docker volume prune -a -f 

permission:
	@echo "🔐 Setting permissions for MinIo data directories ..."
	mkdir -p $(DATA_DIR)
	chmod -R 777 $(DATA_DIR)

# ======================== Deploy ========================  
build:
	docker compose -f docker/minio.docker-compose.yml build
	
up:permission
	@echo "🐳 Starting MinIo containers ..."
	docker compose -f $(FILE) up --force-recreate -d --build 
	@echo "✅ MinIo is up and running."

down:
	@echo "🛑 Stopping MinIo containers ..."
	docker compose -f $(FILE) down
	@echo "✅ MinIo stopped."

restart: down up
	@echo "🔁 Restarted MinIo successfully."

# ======================== logs ========================  
ps:
	@echo "📋 Container status:"
	docker ps --filter "name=minio"
	
logs:
	docker compose -f $(FILE) logs -f