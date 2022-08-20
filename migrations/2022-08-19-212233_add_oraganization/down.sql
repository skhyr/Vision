DROP TABLE organizations;
ALTER TABLE users DROP COLUMN organization_id;
ALTER TABLE users DROP CONSTRAINT users_organization_id_fkey;