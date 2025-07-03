if [ -f .env ]; then
    # Remove comments and empty lines, then export each variable
    export $(sed '/^#/d;/^\s*$/d' .env | xargs)
fi

sqlx migrate run