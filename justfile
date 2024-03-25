set dotenv-load

setup:
    just backend/create-local-db

serve:
    just backend/serve

create-local-db:
    docker run \
        --name native-app-experiment-db \
        -p 5432:5432 \
        -e POSTGRES_PASSWORD=password \
        -e POSTGRES_DB=native_app_experiment \
        -d postgres