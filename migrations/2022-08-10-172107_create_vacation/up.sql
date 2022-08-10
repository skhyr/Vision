CREATE TABLE vacations (
   id uuid NOT NULL PRIMARY KEY,
   user_id uuid NOT NULL,
   start_date DATE NOT NULL,
   end_date DATE NOT NULL,
   title VARCHAR NOT NULL,
   CONSTRAINT fk_vacations FOREIGN KEY(user_id) REFERENCES users(id)
);