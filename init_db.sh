IFS=',' read -r -a databases <<< "$DATABASES"

echo "Databases: $DATABASES"

# Loop through each item in the array
for database in "${databases[@]}"; do
    echo "Creating database: $database"
    PGPASSWORD="$POSTGRES_PASSWORD" createdb -O "$POSTGRES_USER" -h postgres -p 5432 -U "$POSTGRES_USER" -w "$database"
done

exit 0
