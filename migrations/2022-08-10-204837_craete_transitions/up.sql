CREATE TABLE transitions (
   id uuid NOT NULL PRIMARY KEY,
   user_id uuid NOT NULL,
   date DATE NOT NULL,
   CONSTRAINT fk_transitions FOREIGN KEY(user_id) REFERENCES users(id)
);