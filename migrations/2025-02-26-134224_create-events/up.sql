CREATE TABLE events (
    created_at timestamp with time zone PRIMARY KEY,
    user_id int NOT NULL,
    kind varchar NOT NULL,
    x_ft float NOT NULL,
    y_ft float NOT NULL,
    duration_s float NOT NULL,
    impressions int NOT NULL
);
