-- ```sql
-- CREATE TABLE items (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT manage_updated_at('items');
-- ```

CREATE OR REPLACE FUNCTION manage_updated_at(t regclass) RETURNS VOID AS $$
BEGIN
  EXECUTE format('CREATE TRIGGER set_updated_at_trigger BEFORE UPDATE ON %s
                  FOR EACH ROW EXECUTE PROCEDURE set_updated_at()', t);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS $$
BEGIN
  IF (
    NEW IS DISTINCT FROM OLD
    AND NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
  ) THEN
    NEW.updated_at := NOW();
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
