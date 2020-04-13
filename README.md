# Lunch and learn - Techso
#### Exécuter une app full-stack sur docker, localement

## Étapes pour exécuter

1. Déployer la base de données `sh ./database/construct.sh`
2. Construire le backend 

    `cd backend && docker build . -t lunch_and_learn:backend` 

3. Exécuter le backend
    
    ```bash
   docker run 
        --rm \                      # Enlever le conteneur automatiquement 
        -d \                        # Exécuter en arrière-plan 
        -p '127.0.0.1:8000':8000 \  # Forward le port 8000 du docker vers 127.0.0.1:8000 de la machine hôte
        -e SQL_HOST=mysql_techso \  # Variable d'environnement SQL_HOST, utilisation du docker-network dns
        -e SQL_PORT=3306         \  # Variable d'envrionnement
        --network fullstack      \  # Ajoute le conteneur au network fullstack
        --name backend           \  # Nom amical pour les humains
        lunch_and_learn:backend`    # Nom de l'image
    
4. Exécuter le frontend

    `cd frontend && yarn install && yarn start`
    
5. Visitez `localhost:4200`
6. ???
7. Profit