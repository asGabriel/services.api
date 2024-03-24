-- Add migration script here
CREATE TABLE IF NOT EXISTS worknote(
work_note_id UUID PRIMARY KEY,
category VARCHAR(255) NOT NULL,
work_date DATE NOT NULL,
work_hours FLOAT NOT NULL,
observation VARCHAR(255),
created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
updated_at TIMESTAMPTZ DEFAULT NULL,
deleted_at TIMESTAMPTZ DEFAULT NULL
);