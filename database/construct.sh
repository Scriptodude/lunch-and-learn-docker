# Fichier de construction de l'environnement docker (Sans docker compose) de la base de données
# pour le lunch & learn de techso (www.techso.com)
# Autheur: Jonathan Lavigne (jlavigne@techso.com)
# Utilisation: ./construct.sh

# Dossier courant du script
SCRIPT=$(readlink -f "$0")
SCRIPTDIR=$(dirname "$SCRIPT")

# Mot de passe du root dans le docker
password="root"

# Destination du fichier dump.sql dans le docker
dump_dest="/tmp/dump.sql"

# Commande pour executer le dump sur le docker
dump_cmd="mysql -u root -p$password < $dump_dest"

# Nom du network auquel se connecter
network_name="fullstack"

# Nom du conteneur docker
container_name="mysql_techso"

# Step 1 - Construire le docker network
docker network rm "$network_name" || true
docker network create "$network_name"

# Step 2 - Construire l'image docker
docker container rm "$container_name" -f || true
docker run \
  --name "$container_name" \
  -e MYSQL_ROOT_PASSWORD="$password" \
  -e LANG=C.UTF-8 \
  --network "$network_name" \
  -d \
  -p '127.0.0.1:1337':3306 \
  mariadb:latest

# Step 3 - Add the migration script
echo "Copie en cours..."
docker cp "$SCRIPTDIR/dump.sql" "$container_name":"$dump_dest";

echo "exécution du dump '$dump_cmd'"
until docker exec "$container_name" sh -c "$dump_cmd"
do
  echo "Tentative échouée, nouvelle tentative dans 5 secondes...";
  sleep 5
done

echo "Succès !"
# Step 4 - Profit