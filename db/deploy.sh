SQL_SCHEMA=$1
DATABASE_NAME=$2
SQL_USER=$3

# Generate database if not exists
if [ "$( psql -XtAc "SELECT 1 FROM pg_database WHERE datname='$DATABASE_NAME'" )" != '1' ]
then
    psql -XtAc "CREATE DATABASE $DATABASE_NAME WITH OWNER = $SQL_USER"
    psql -d $DATABASE_NAME -f $SQL_SCHEMA -U $SQL_USER
fi

# Apply migrations

