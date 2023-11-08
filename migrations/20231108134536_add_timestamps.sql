-- Add a new column for the creation date
ALTER TABLE tickets
ADD COLUMN created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL;

-- Add an optional column for the deadline date
ALTER TABLE tickets
ADD COLUMN deadline_date TIMESTAMPTZ;
