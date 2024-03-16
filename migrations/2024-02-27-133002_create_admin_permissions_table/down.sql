-- This file should undo anything in `up.sql`

-- Query to retrieve the name of the foreign key constraint
DO $$
DECLARE
    fk_name text;
BEGIN
    SELECT conname INTO fk_name
    FROM pg_constraint
    WHERE conrelid = 'admin_permissions'::regclass
      AND contype = 'f';

    -- Check if a foreign key constraint was found
    IF fk_name IS NOT NULL THEN
        -- Drop the foreign key constraint
        EXECUTE 'ALTER TABLE admin_permissions DROP CONSTRAINT ' || quote_ident(fk_name);
    END IF;
END $$;

-- Now that the foreign key constraint is dropped, you can drop the table
DROP TABLE admin_permissions;
