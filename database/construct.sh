# Fichier de construction de l'environnement docker (Sans docker compose) de la base de données
# pour le lunch & learn de techso (www.techso.com)
# Autheur: Jonathan Lavigne (jlavigne@techso.com)
# Utilisation: ./construct.sh

# Nom du network auquel se connecter
network_name="fullstack"

# Step 1 - Construire le docker network, s'il n'existe pas
docker network inspect "$network_name" >/dev/null 2>&1 || \
    docker network create "$network_name"

# Step 2 - Construire l'image docker
docker run \
  --name mysql_techso \
  -e MYSQL_ROOT_PASSWORD=root \
  --network "$network_name" \
  -d \
  -p 1337:3306 \
  mariadb:latest

# Step 3 - ???????
# Step 4 - Profit