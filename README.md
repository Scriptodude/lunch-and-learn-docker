# Lunch and learn - Techso
#### Exécuter une app full-stack sur docker, localement

La version de docker est `19.03.12, build 48a66213fe`
La version de docker-compose est 
```
docker-compose version 1.25.4, build unknown
docker-py version: 4.0.2
CPython version: 3.7.7
OpenSSL version: OpenSSL 1.1.1g FIPS  21 Apr 2020
```

## Étapes pour exécuter

1. Déployer la base de données `sh ./database/construct.sh`
2. Construire le backend 

    `cd backend-rust && docker build . -t lunch_and_learn:backend` 

3. Exécuter le backend
    
    `docker run --rm -d -p '127.0.0.1:8000':8000 -e SQL_HOST=mysql_techso -e SQL_PORT=3306 --network fullstack --name backend lunch_and_learn:backend`
    
    Explications: 
    ```bash
   docker run \
        --rm \                      # Enlever le conteneur automatiquement 
        -d \                        # Exécuter en arrière-plan 
        -p '127.0.0.1:8000':8000 \  # Forward le port 8000 du docker vers 127.0.0.1:8000 de la machine hôte
        -e SQL_HOST=mysql_techso \  # Variable d'environnement SQL_HOST, utilisation du docker-network dns
        -e SQL_PORT=3306         \  # Variable d'envrionnement
        --network fullstack      \  # Ajoute le conteneur au network fullstack
        --name backend           \  # Nom amical pour les humains
        lunch_and_learn:backend    # Nom de l'image
    ```
   
4. Exécuter le frontend

    `cd frontend && yarn install && yarn start`
    
5. Visitez `localhost:4200`
6. ???
7. Profit

## Docker compose
La version de docker-compose utilisée est 1.25

1. `docker-compose up`
2. Lancer le frontend `cd frontend && yarn install && yarn start`
3. Visitez `localhost:4200`