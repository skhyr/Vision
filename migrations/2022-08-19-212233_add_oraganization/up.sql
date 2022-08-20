CREATE TABLE organizations (
   id uuid NOT NULL PRIMARY KEY,
   name VARCHAR NOT NULL
);
ALTER TABLE users ADD COLUMN organization_id uuid NOT NULL;
ALTER TABLE users ADD FOREIGN KEY (organization_id) REFERENCES organizations(id);